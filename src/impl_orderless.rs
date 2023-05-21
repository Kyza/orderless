use darling::FromMeta;

use darling::export::NestedMeta;
use proc_macro::TokenStream;

use proc_macro_error::abort;
use quote::{quote, ToTokens};

use syn::Ident;
use syn::{parse_macro_input, ImplItem, ItemImpl, Meta};

use crate::utils::DIdent;

#[derive(Debug, Clone, FromMeta)]
struct ImplOrderlessOptions {
	pub name: Option<DIdent>,
}

pub fn impl_orderless(attr: TokenStream, item: TokenStream) -> TokenStream {
	let options = match NestedMeta::parse_meta_list(attr.into()) {
		Ok(v) => v,
		Err(e) => {
			return TokenStream::from(darling::Error::from(e).write_errors());
		}
	};
	let options: ImplOrderlessOptions =
		match ImplOrderlessOptions::from_list(&options) {
			Ok(v) => v,
			Err(e) => {
				return TokenStream::from(e.write_errors());
			}
		};

	let mut implementation = parse_macro_input!(item as ItemImpl);

	let impl_name: Ident = {
		if let Some(name) = options.name {
			name.0
		} else {
			let impl_name =
				&implementation.clone().self_ty.to_token_stream().to_string();
			let Ok(impl_name) = Ident::from_string(impl_name) else {
			abort! { impl_name,
					"failed to automatically convert function path into an identifier";
					help = "¯\\_(ツ)_/¯";
				}
			};
			impl_name
		}
	};

	let mut creates: Vec<proc_macro2::TokenStream> = vec![];

	// Collect the functions that have make_orderless used on them, remove the attribute, and make a `create_orderless!` for it.
	for item in implementation.items.iter_mut() {
		let func = if let ImplItem::Fn(func) = item {
			func
		} else {
			continue;
		};

		let func_name = &func.sig.ident;

		for (i, attr) in func.clone().attrs.iter().enumerate() {
			if attr.path().is_ident("make_orderless") {
				// Remove the attribute since macros can't be defined inside of `impl` blocks.
				func.attrs.remove(i);

				// Get the args.
				let args = if let Meta::List(meta) = attr.meta.clone() {
					meta.tokens
				} else {
					quote! {}
				};

				// Make a new `create_orderless!`.
				creates.push(quote! {
					::orderless::create_orderless! {
						func = #impl_name::#func_name,
						#args
					}
				});
			}
		}
	}

	quote! {
		#implementation
		#(#creates)*
	}
	.into()
}
