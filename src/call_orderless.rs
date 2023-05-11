use darling::{export::NestedMeta, FromMeta};
use proc_macro::TokenStream;

use proc_macro_error::abort;
use quote::quote;
use syn::{Expr, ExprPath, Ident};

use indexmap::IndexMap;

use crate::utils::ArgsIndexMap;

#[derive(Debug, Clone, FromMeta)]
pub struct CallOrderlessOptions {
	func: ExprPath,
	defs: ArgsIndexMap,
	args: ArgsIndexMap,
}

pub fn call_orderless(input: TokenStream) -> TokenStream {
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
	let mut args: IndexMap<Ident, Option<Expr>> = IndexMap::new();

	for (name, value) in options.defs.0.clone() {
		args.insert(name, value);
	}

	for (name, value) in options.args.0 {
		if args.contains_key(&name) {
			args.insert(name, value);
		} else {
			abort! { name,
				"orderless function was called with an extra \
				 argument";
				help = "Remove the extra argument or define it if it exists.";
			}
		}
	}

	// Error if there's a missing argument.
	for (name, value) in &args {
		if value.is_none() {
			abort! { name,
				"orderless function was called while missing an \
				 argument";
				help = "Remove the extra argument or define it if it exists.";
			}
		}
	}
	let args = args.into_values();

	quote! {
		#func(#(#args),*)
	}
	.into()
}
