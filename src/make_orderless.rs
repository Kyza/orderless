use darling::{export::NestedMeta, FromMeta};
use indexmap::IndexMap;
use proc_macro::TokenStream;


use quote::quote;

use syn::{parse_macro_input, ItemFn};
use syn::{Expr, Ident};

use crate::utils::DIdent;
use crate::utils::{ident_from_fn_arg, ArgsIndexMap};

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
		arg_map.insert(ident_from_fn_arg(arg), None);
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
