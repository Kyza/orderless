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

## How does it work?

### `call_orderless!`

`call_orderless!` is the proc macro that does all the heavy lifting. It takes a bunch of info such as the function's name, the order of the arguments, and the default values.

```rs
call_orderless! {
	func = two,
	order(a, b),
	defs(a = false, b = false),
	args(a = true, b = false),
}
```

As you can see, using it on its own is pretty pointless. But it's perfect for other macros to pass info they have to it.

### `create_orderless!`

`create_orderless!` is another helper macro. It simplifies the process of writing `call_orderless!` by generating a `macro_rules!` macro which has most of the info built in. 

```rs
create_orderless! {
	func = two,
	order(a, b),
	defs(a = false, b = false)
}

// Generates...
// Note `order(...)` disappears because it's integrated into `defs(...)` by `create_orderless!`.
macro_rules! two {
	( $($arg_name:ident $(= $arg_value:expr)?),*$(,)? ) => {
		::orderless::call_orderless!(
			func = two,
			defs(a = false, b = false),
			args($($arg_name $(= $arg_value)?),*),
		)
	};
	() => {
		::orderless::call_orderless!(
			func = two,
			defs(a = false, b = false),
			args(),
		)
	};
}

// Called like...
two!(b = true);
```

Now you have a function-like macro which can be used very simply.

### `make_orderless`

`make_orderless` is an attribute macro which simplifies the process *even more* by grabbing info already available in the function's definition.

```rs
#[make_orderless(defs(a = false, b = false))]
fn two<T>(a: T, b: T) -> (T, T) {
	(a, b)
}

// Generates the same thing as `create_orderless!`...
```

### `impl_orderless`

The main problem with `make_orderless` is that since it generates a `macro_rules!` *right there*, it can't be used inside of `impl` blocks.

```rs
struct Args {}

impl Args {
	#[make_orderless(defs(a = false, b = false))] // ERROR!!
	pub fn two(a: bool, b: bool) -> (bool, bool) {
		(a, b)
	}
}
```

Fortunately, the `impl_orderless` macro makes this possible.

```rs
struct Args {}

#[impl_orderless]
impl Args {
	#[make_orderless(defs(a = false, b = false))] // SUCCESS!!
	pub fn two(a: bool, b: bool) -> (bool, bool) {
		(a, b)
	}
}
```

It does this by removing all the `make_orderless` attributes and converting them into `create_orderless!` outside of the `impl` block.

With all this chaining it's macro-ception. A macro that converts a macro to another macro, which creates a macro, which calls a macro. But in the end this is all compile-time and doesn't impact runtime performance at all. `two!()` simply compiles to `two(false, false)`!
