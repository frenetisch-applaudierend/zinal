// use crate::{Children, Context, Escaper, Renderable};

// #[allow(missing_docs)]
// pub trait RenderExpressionRenderable {
//     fn render_expr(
//         &self,
//         writer: &mut dyn std::fmt::Write,
//         escaper: &dyn Escaper,
//         context: &Context,
//     ) -> Result<(), std::fmt::Error>;
// }

// #[allow(missing_docs)]
// pub trait RenderExpressionChildren {
//     fn render_expr(
//         self,
//         writer: &mut dyn std::fmt::Write,
//         escaper: &dyn Escaper,
//         context: &Context,
//     ) -> Result<(), std::fmt::Error>;
// }

// impl<T> RenderExpressionRenderable for T
// where
//     T: Renderable,
// {
//     fn render_expr(
//         &self,
//         writer: &mut dyn std::fmt::Write,
//         escaper: &dyn Escaper,
//         _context: &Context,
//     ) -> Result<(), std::fmt::Error> {
//         Renderable::render(self, writer, escaper)
//     }
// }

// impl RenderExpressionChildren for Children {
//     fn render_expr(
//         self,
//         writer: &mut dyn std::fmt::Write,
//         escaper: &dyn Escaper,
//         context: &Context,
//     ) -> Result<(), std::fmt::Error> {
//         Children::render(self, writer, escaper, context)
//     }
// }
