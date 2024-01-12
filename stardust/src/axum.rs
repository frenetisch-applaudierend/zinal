use crate::{content_type::Html, Template};

impl<T> axum_core::response::IntoResponse for T
where
    T: Template<Html>,
{
    fn into_response(self) -> axum_core::response::Response {
        todo!()
    }
}
