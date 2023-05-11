use darling::{export::NestedMeta, FromMeta};
use indexmap::IndexMap;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::abort;
use quote::quote;

use syn::{parse_macro_input, FnArg, ItemFn, Pat};
use syn::{Expr, Ident};

use crate::utils::ArgsIndexMap;
use crate::utils::DIdent;

#[derive(Debug, Clone, FromMeta)]
pub struct MakeOrderlessOptions {
	name: Option<DIdent>,
	public: Option<bool>,
	defs: Option<ArgsIndexMap>,
}

pub fn make_orderless(attr: TokenStream, item: TokenStream) -> TokenStream {
	let options = match NestedMeta::parse_meta_list(attr.into()) {
		Ok(v) => v,
		Err(e) => {
			return TokenStream::from(darling::Error::from(e).write_errors());
		}
	};
	let options = match MakeOrderlessOptions::from_list(&options) {
		Ok(v) => v,
		Err(e) => {
			return TokenStream::from(e.write_errors());
		}
	};

	let func = parse_macro_input!(item as ItemFn);
	let macro_name = if let Some(name) = options.name {
		name.0
	} else {
		func.clone().sig.ident
	};
	let func_name = func.clone().sig.ident;

	// Get all the arguments on the function itself.
	let mut arg_map: IndexMap<Ident, Option<Expr>> = IndexMap::new();
	for arg in &func.sig.inputs {
		match arg {
			FnArg::Typed(arg) => {
				let Pat::Ident(arg_name) = *arg.pat.clone() else {
					abort! { arg,
						"the argument was not an identifier";
						help = "I don't know how you got here, or how this could\
						 possibly happen, but apparently it can. So I'm writing\
						 this message to say that you're doing something wrong\
						 here... Maybe... Or maybe I am. I have no idea.";
					}
				};
				arg_map.insert(arg_name.ident, None);
			}
			FnArg::Receiver(_) => {
				arg_map.insert(Ident::new("self", Span::call_site()), None);
			}
		}
	}

	let defs = options.defs;
	if let Some(defs) = defs {
		for (arg_name, arg_value) in defs.0 {
			arg_map.insert(arg_name, arg_value);
		}
	}

	let defs: Vec<_> = arg_map
		.iter()
		.map(|(name, value)| {
			if let Some(value) = value {
				quote! {
					#name = #value
				}
			} else {
				quote! {
					#name
				}
			}
		})
		.collect();

	let public = if let Some(public) = options.public {
		public
	} else {
		false
	};

	quote! {
		#func
		::orderless::create_orderless! {
			name = #macro_name,
			public = #public,
			func = #func_name,
			defs(#(#defs),*)
		}
	}
	.into()
}
