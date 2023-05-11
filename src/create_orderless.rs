use darling::util::path_to_string;
use darling::{export::NestedMeta, FromMeta};
use indexmap::IndexMap;
use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;

use syn::{Expr, ExprPath, Ident};

use crate::utils::ArgsIndexMap;
use crate::utils::DIdent;

#[derive(Debug, Clone, FromMeta)]
pub struct CallOrderlessOptions {
	name: Option<DIdent>,
	public: Option<bool>,
	func: ExprPath,
	defs: ArgsIndexMap,
}

pub fn create_orderless(input: TokenStream) -> TokenStream {
	let options = match NestedMeta::parse_meta_list(input.into()) {
		Ok(v) => v,
		Err(e) => {
			return TokenStream::from(darling::Error::from(e).write_errors());
		}
	};
	let options = match CallOrderlessOptions::from_list(&options) {
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

	let public = if let Some(public) = options.public {
		if public {
			quote! {
				#[macro_export]
			}
		} else {
			quote! {}
		}
	} else {
		quote! {}
	};

	quote! {
		#public
		macro_rules! #name {
			( $($arg_name:ident $(= $arg_value:expr)?),* ) => {
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
	}
	.into()
}
