use darling::util::path_to_string;
use darling::{export::NestedMeta, FromMeta};
use indexmap::IndexMap;
use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;



use syn::{Expr, ExprPath, Ident};

use crate::utils::{ArgsIndexMap};
use crate::utils::{DIdent, IdentIndexSet};

#[derive(Debug, Clone, FromMeta)]
pub struct CreateOrderlessOptions {
	name: Option<DIdent>,
	public: Option<bool>,
	func: ExprPath,
	order: Option<IdentIndexSet>,
	defs: ArgsIndexMap,
}

pub fn create_orderless(input: TokenStream) -> TokenStream {
	let options = match NestedMeta::parse_meta_list(input.into()) {
		Ok(v) => v,
		Err(e) => {
			return TokenStream::from(darling::Error::from(e).write_errors());
		}
	};
	let options = match CreateOrderlessOptions::from_list(&options) {
		Ok(v) => v,
		Err(e) => {
			return TokenStream::from(e.write_errors());
		}
	};

	let func = options.func;
	let mut name = if let Some(name) = options.name {
		name.0.to_string()
	} else {
		path_to_string(&func.path)
	};
	name = name.replace(':', "_");
	let Ok(name) = Ident::from_string(&name) else {
		abort! { name,
			"failed to automatically convert function path into an identifier";
			help = "¯\\_(ツ)_/¯";
		}
	};
	let mut args: IndexMap<Ident, Option<Expr>> = IndexMap::new();

	// Preserve the order if the args are provided.
	if let Some(sig) = options.order {
		for ident in sig.0.into_iter() {
			args.insert(ident, None);
		}
	}

	for (name, value) in options.defs.0 {
		args.insert(name, value);
	}
	let args: Vec<_> = args
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

	let before_public;
	let after_public;
	if let Some(public) = options.public {
		if public {
			before_public = quote! {
				#[macro_export]
			};
			after_public = quote! {
				pub use #name;
			};
		} else {
			before_public = quote! {};
			after_public = quote! {};
		}
	} else {
		before_public = quote! {};
		after_public = quote! {};
	};

	quote! {
		#before_public
		macro_rules! #name {
			( $($arg_name:ident $(= $arg_value:expr)?),*$(,)? ) => {
				::orderless::call_orderless!(
					func = #func,
					defs(#(#args),*),
					args($($arg_name $(= $arg_value)?),*),
				)
			};
			() => {
				::orderless::call_orderless!(
					func = #func(#(#args),*),
					defs(#(#args),*),
					args(),
				)
			};
		}
		#after_public
	}
	.into()
}
