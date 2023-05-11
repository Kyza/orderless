use darling::{export::NestedMeta, FromMeta};
use indexmap::IndexMap;
use proc_macro_error::abort;

use syn::{parse_quote, Expr, Ident, Meta, Path};

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
