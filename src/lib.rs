#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]

// #![warn(missing_docs)]

use proc_macro::TokenStream as BaseTokenStream;

#[proc_macro_derive(Foo, attributes(foo))]
pub fn derive_potatoes(input: BaseTokenStream) -> BaseTokenStream {
  todo!()
}

