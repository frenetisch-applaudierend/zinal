use zinal::*;

#[test]
pub fn required_params() {
    #[derive(Template)]
    #[template("<Inner />")]
    struct Outer {
        #[provide_context]
        count: u8,
    }

    #[derive(Template)]
    #[template("{{self.count}}")]
    struct Inner<'a> {
        #[from_context]
        count: &'a u8,
    }

    let rendered = render(|| Outer { count: 10 });
    assert_eq!(rendered, "10");
}

fn render<T: Template>(builder: impl FnOnce() -> T) -> String {
    builder()
        .render_to_string()
        .expect("should render without error")
}
