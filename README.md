# orderless!

`orderless` generates macros for you that allow you to use orderless/named functions in Rust.

```rs
#[make_orderless(defs(a = 2))]
fn add(a: usize, b: usize) -> usize {
	a + b
}

// Compiles to add(2, 2) for no runtime performance hit!
add!(b = 2); // 4
```

## Features

- [x] Attribute macro.
- [x] Procedural macro.
- [x] Paths to functions (functions from crates and `impl`).
- [x] Default argument values.
	- [x] Identifiers.
	- [x] Expressions.
	- [x] `const` and `static` variables.
	- [x] Optionally don't provide a default value.
- [x] Shortcut identical name and value to just the name. `a = a` to `a`.
- [x] Attribute macro `impl_orderless` for `make_orderless` in `impl` blocks.

## Docs

Documentation is provided on [docs.rs](https://docs.rs/orderless).
