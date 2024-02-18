use std::borrow::Cow;
use std::ops::Deref;

use crate::Escaper;

/// Implemented by values that can be rendered to a template.
pub trait Renderable {
    /// Render the value to the writer, escaping it as needed
    /// with the provided escaper.
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error>;
}

// TODO: Enable this once impl specialization is available on stable Rust

// impl<T> Renderable for T
// where
//     T: std::fmt::Display,
// {
//     fn render(
//         &self,
//         writer: &mut dyn std::fmt::Write,
//         escaper: &dyn Escaper,
//     ) -> Result<(), std::fmt::Error> {
//         let raw = format!("{}", self);
//         let escaped = escaper.escape(Cow::Owned(raw));

//         write!(writer, "{}", escaped)
//     }
// }

macro_rules! render_unescaped {
    ($t:ty) => {
        impl Renderable for $t {
            fn render(
                &self,
                writer: &mut dyn std::fmt::Write,
                _: &dyn Escaper,
            ) -> Result<(), std::fmt::Error> {
                write!(writer, "{}", self)
            }
        }
    };
}

macro_rules! render_deref {
    ($t:ty) => {
        impl<T> Renderable for $t
        where
            T: Renderable,
        {
            fn render(
                &self,
                writer: &mut dyn std::fmt::Write,
                escaper: &dyn Escaper,
            ) -> Result<(), std::fmt::Error> {
                Renderable::render(self.deref(), writer, escaper)
            }
        }
    };
}

render_unescaped!(bool);

render_unescaped!(u8);
render_unescaped!(u16);
render_unescaped!(u32);
render_unescaped!(u64);
render_unescaped!(u128);
render_unescaped!(&u8);
render_unescaped!(&u16);
render_unescaped!(&u32);
render_unescaped!(&u64);
render_unescaped!(&u128);

render_unescaped!(i8);
render_unescaped!(i16);
render_unescaped!(i32);
render_unescaped!(i64);
render_unescaped!(i128);
render_unescaped!(&i8);
render_unescaped!(&i16);
render_unescaped!(&i32);
render_unescaped!(&i64);
render_unescaped!(&i128);

render_deref!(std::boxed::Box<T>);
render_deref!(std::rc::Rc<T>);
render_deref!(std::sync::Arc<T>);

impl Renderable for &str {
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error> {
        write!(writer, "{}", escaper.escape(Cow::Borrowed(self)))
    }
}

impl Renderable for String {
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error> {
        write!(writer, "{}", escaper.escape(Cow::Borrowed(self)))
    }
}

impl<'a> Renderable for Cow<'a, str> {
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error> {
        write!(writer, "{}", escaper.escape(Cow::Borrowed(self)))
    }
}

impl<T> Renderable for Option<T>
where
    T: Renderable,
{
    fn render(
        &self,
        writer: &mut dyn std::fmt::Write,
        escaper: &dyn Escaper,
    ) -> Result<(), std::fmt::Error> {
        match self {
            Some(r) => Renderable::render(r, writer, escaper),
            None => Ok(()),
        }
    }
}
