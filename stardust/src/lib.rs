pub mod content_type;

mod children;
mod context;
mod renderable;
mod template;

#[cfg(feature = "axum")]
mod axum;

pub use children::*;
pub use context::*;
pub use renderable::*;
pub use template::*;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use stardust_derive::Template;

#[cfg(test)]
mod tests;
