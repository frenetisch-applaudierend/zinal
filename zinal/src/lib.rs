#![deny(unsafe_code)]

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
