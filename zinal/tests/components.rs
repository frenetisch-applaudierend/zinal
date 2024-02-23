use zinal::*;

#[test]
fn nested_components() {
    #[derive(Template)]
    #[template(
        "<Middle bool_prop={{self.bool_prop}}>= <Inner bool_prop={{self.bool_prop}} /></Middle>"
    )]
    struct Outer {
        bool_prop: bool,
    }

    #[derive(Template)]
    #[template("{{self.bool_prop}} {{@children}}")]
    struct Middle {
        bool_prop: bool,
    }

    #[derive(Template)]
    #[template("{{self.bool_prop}}")]
    struct Inner {
        bool_prop: bool,
    }

    assert_eq!(
        Ok(String::from("true = true")),
        Outer { bool_prop: true }.render_to_string()
    );
    assert_eq!(
        Ok(String::from("false = false")),
        Outer { bool_prop: false }.render_to_string()
    );
}
