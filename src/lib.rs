#![doc = include_str!("../README.md")]
#![allow(clippy::tabs_in_doc_comments)]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod call_orderless;
mod create_orderless;
mod impl_orderless;
mod make_orderless;
mod utils;

#[proc_macro]
#[proc_macro_error]
/// An internal procedural macro that calls a function based on a definition.
///
/// This macro is called by the other macros.
///
/// I don't recommend using this manually.
pub fn call_orderless(input: TokenStream) -> TokenStream {
	call_orderless::call_orderless(input)
}

#[proc_macro_attribute]
#[proc_macro_error]
/// A procedural macro that's meant for creating macros that run `call_orderless!` with the provided definition.
///
/// ```rs
/// #[make_orderless(defs(a = 2))]
/// fn add(a: usize, b: usize) -> usize {
/// 	a + b
/// }
///
/// // Compiles to add(2, 2) for no runtime performance hit!
/// add!(b = 2); // 4
/// ```
/// ### Named
///
/// The generated macro can be named whatever you want.
///
/// ```rs
/// #[make_orderless(name = cool_add, defs(a = 2, b))]
/// fn add(a: usize, b: usize) -> usize {
/// 	a + b
/// }
///
/// // Compiles to add(2, 2) for no runtime performance hit!
/// cool_add!(b = 2); // 4
/// ```
/// ### Expressions
///
/// You can also use expressions as the default argument values, but keep in mind that every time the function is called it will rerun that expression.
///
/// ```rs
/// #[make_orderless(
/// 	name = cool_add,
/// 	defs(a = {
/// 		add(2, 2) + add(2, 2)
/// 	}, b)
/// )]
/// fn add(a: usize, b: usize) -> usize {
/// 	a + b
/// }
/// ```
///
/// To avoid slow performance you can pass const/static variables, but you'll likely have to convert them to the proper type in an expression.
///
/// ### Public
///
/// You can export the generated macro by setting `public` to true.
///
/// ```rs
/// #[make_orderless(public = true)]
/// fn add(a: usize, b: usize) -> usize {
/// 	a + b
/// }
/// ```
///
/// ### `impl`
///
/// ```rs
/// struct Three<T> {
/// 	a: T,
/// 	b: T,
/// }
///
/// // This is required in order for `make_orderless` to work inside it.
/// // You can choose to override the name as well.
/// #[impl_orderless(name = Args)]
/// impl<T> Three<T> {
/// 	#[make_orderless(
/// 		defs(self = Three {
/// 			a: false,
/// 			b: false
/// 		}, c)
/// 	)]
/// 	pub fn three(self, c: T) -> (T, T, T) {
/// 		(self.a, self.b, c)
/// 	}
/// }
///
/// Args__three!(c = true); // (false, false, true)
/// ```
pub fn make_orderless(attr: TokenStream, item: TokenStream) -> TokenStream {
	make_orderless::make_orderless(attr, item)
}

#[proc_macro]
#[proc_macro_error]
/// A procedural macro that's meant for creating macros that run `call_orderless!` with the provided definition.
///
/// ```rs
/// create_orderless! {
/// 	func = add,
/// 	defs(a = 2, b)
/// }
///
/// // Compiles to add(2, 2) for no runtime performance hit!
/// add!(b = 2); // 4
/// ```
/// ### Named
///
/// The generated macro can be named whatever you want.
///
/// ```rs
/// create_orderless! {
/// 	name = cool_add,
/// 	func = add,
/// 	defs(a = 2, b)
/// }
///
/// // Compiles to add(2, 2) for no runtime performance hit!
/// cool_add!(b = 2); // 4
/// ```
/// ### Expressions
///
/// You can also use expressions as the default argument values, but keep in mind that every time the function is called it will rerun that expression.
///
/// ```rs
/// create_orderless! {
/// 	func = add,
/// 	defs(a = {
/// 		add(2, 2) + add(2, 2)
/// 	}, b)
/// }
/// ```
///
/// To avoid slow performance you can pass const/static variables, but you'll likely have to convert them to the proper type in an expression.
///
/// ### Public
///
/// You can export the generated macro by setting `public` to true.
///
/// ```rs
/// create_orderless! {
/// 	name = add,
/// 	func = add,
/// 	public = true,
/// 	defs(a = 2, b = 2)
/// }
/// ```
///
/// ### Paths
///
/// It's also possible to use paths to functions for use with `impl` functions or functions deep inside crates.
/// ```rs
/// struct Three<T> {
/// 	a: T,
/// 	b: T,
/// }
///
/// impl<T> Three<T> {
/// 	pub fn three(self, c: T) -> (T, T, T) {
/// 		(self.a, self.b, c)
/// 	}
/// }
///
/// create_orderless! {
/// 	func = Three::three,
/// 	defs(self = Three {
/// 		a: false,
/// 		b: false
/// 	}, c)
/// }
///
/// Three__three!(c = true); // (false, false, true)
/// ```
pub fn create_orderless(input: TokenStream) -> TokenStream {
	create_orderless::create_orderless(input)
}

#[proc_macro_attribute]
#[proc_macro_error]
/// A procedural macro that allows `make_orderless` to be used in `impl` blocks.
///
/// ```rs
/// struct Three<T> {
/// 	a: T,
/// 	b: T,
/// }
///
/// // This is required in order for `make_orderless` to work inside it.
/// // You can choose to override the name as well.
/// #[impl_orderless(name = Args)]
/// impl<T> Three<T> {
/// 	#[make_orderless(
/// 		defs(self = Three {
/// 			a: false,
/// 			b: false
/// 		}, c)
/// 	)]
/// 	pub fn three(self, c: T) -> (T, T, T) {
/// 		(self.a, self.b, c)
/// 	}
/// }
///
/// Args__three!(c = true); // (false, false, true)
/// ```
pub fn impl_orderless(attr: TokenStream, item: TokenStream) -> TokenStream {
	impl_orderless::impl_orderless(attr, item)
}
