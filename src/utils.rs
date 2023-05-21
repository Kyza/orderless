use darling::{export::NestedMeta, FromMeta};
use indexmap::{IndexMap, IndexSet};
use proc_macro2::Span;
use proc_macro_error::abort;

use quote::ToTokens;
use syn::{parse2, parse_quote, Expr, FnArg, Ident, Meta, Pat, Path};

pub fn strict_ident_from_path(path: Path) -> Ident {
	// It could be an actual path, so test for that.
	if path.segments.len() > 1 {
		abort! { path,
			"expected an identifier";
			help = "This should not be a path.";
		}
	}
	path.get_ident()
		.unwrap_or_else(|| {
			abort! { path,
				"expected an identifier";
				help = "This path has no identifier.";
			}
		})
		.clone()
}

#[derive(Debug, Clone)]
pub struct DIdent(pub Ident);

impl FromMeta for DIdent {
	fn from_expr(expr: &Expr) -> darling::Result<Self> {
		let Expr::Path(path) = expr else {
			abort! { expr,
				"expected an identifier";
				help = "This expression is not an identifier.";
			}
		};
		let ident = strict_ident_from_path(path.path.clone());

		Ok(DIdent(ident))
	}
}

#[derive(Debug, Clone)]
pub struct ArgsIndexMap(pub IndexMap<Ident, Option<Expr>>);

impl FromMeta for ArgsIndexMap {
	fn from_list(
		items: &[darling::export::NestedMeta],
	) -> darling::Result<Self> {
		let mut im = IndexMap::new();
		for item in items {
			match item {
				NestedMeta::Meta(Meta::NameValue(item)) => {
					// We don't care what item.eq is. That's up to user preference.
					im.insert(
						strict_ident_from_path(item.path.clone()),
						Some(item.value.clone()),
					);
				}
				NestedMeta::Meta(Meta::Path(item)) => {
					// This is for shortcutting `a = a` to `a`.
					im.insert(
						strict_ident_from_path(item.clone()),
						Some(parse_quote! {#item}),
					);
				}
				_ => {
					abort! { item,
						"expected an identifier";
						help = "This expression is not an identifier.";
					}
				}
			}
		}
		Ok(ArgsIndexMap(im))
	}
}

#[derive(Debug, Clone)]
pub struct IdentIndexSet(pub IndexSet<Ident>);

impl FromMeta for IdentIndexSet {
	fn from_list(
		items: &[darling::export::NestedMeta],
	) -> darling::Result<Self> {
		let mut is = IndexSet::new();
		for item in items {
			// `self` is a path not an identifier.
			let path = parse2::<Path>(item.to_token_stream());
			if let Ok(path) = path {
				is.insert(strict_ident_from_path(path));
			} else {
				abort! { item,
					"expected an identifier";
					help = "This expression is not an identifier.";
				}
			}
		}
		Ok(IdentIndexSet(is))
	}
}

pub fn ident_from_fn_arg(arg: &FnArg) -> Ident {
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
			arg_name.ident
		}
		FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
	}
}
