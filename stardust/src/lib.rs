mod children;
mod context;
mod escaper;
mod renderable;
mod template;

pub mod builder;
#[cfg(feature = "derive")]
pub mod derive;

pub use children::*;
pub use context::*;
pub use escaper::*;
pub use renderable::*;
pub use template::*;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use stardust_derive::Template;

#[cfg(test)]
mod tests;
