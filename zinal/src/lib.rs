//! Zinal is a HTML template rendering library for rust programs.
//!
//! # Example
//!
//! ```rust
//! use zinal::*;
//!
//! #[derive(Template)]
//! #[template(content = "
//!   <div>We greet the following people:</div>
//!   <ul>
//!   <# for name in &self.names #>
//!     <Person name={{name}} />
//!   <#end>
//!   </ul>
//! ")]
//! struct Greetings {
//!   names: Vec<String>
//! }
//!
//! #[derive(Template)]
//! #[template(content = "<li><p>{{self.name}}</p></li>")]
//! struct Person<'a> {
//!   name: &'a str,
//! }
//!
//! let greetings = Greetings {
//!   names: vec!["Mary".to_owned(), "John".to_owned(), "Kate".to_owned(), "Agnes".to_owned()]
//! };
//!
//! println!("{}", greetings.render_to_string().unwrap());
//!
//! // Prints (possibly with some insignificant whitespace differences):
//! // <div>We greet the following people:</div>
//! // <ul>
//! // <li><p>Mary</p></li>
//! // <li><p>John</p></li>
//! // <li><p>Kate</p></li>
//! // <li><p>Agnes</p></li>
//! // </ul>
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod children;
mod context;
mod escaper;
mod renderable;
mod template;

pub mod builder;

pub use children::*;
pub use context::*;
pub use escaper::*;
pub use renderable::*;
pub use template::*;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use zinal_derive::Template;

#[cfg(test)]
mod tests;
