use std::fmt::Error;

use crate::{Children, RenderContext, Renderable, Template};

#[test]
fn hello_world() {
    struct HelloWorld<'a> {
        name: &'a dyn Renderable,
    }

    impl Template for HelloWorld<'_> {
        fn render(&self, context: &mut RenderContext) -> Result<(), std::fmt::Error> {
            context.render_literal("Hello, ")?;
            context.render_renderable(self.name)?;

            Ok(())
        }
    }

    let name = "World!";
    let hello = HelloWorld { name: &name };

    assert_eq!(hello.render_to_string(), Ok("Hello, World!".to_string()));
}

#[test]
fn target_example() {
    // Source:
    //
    // #[derive(Template)]
    // #[template(
    //    type = "html",
    //    content = "<div><Person name="Fred" age={35}><p>Lorem ipsum...</p></Person></div>"
    // )]
    // struct Info;
    //
    // #[derive(Template)]
    // #[template(
    //    type = "html",
    //    content = "<p>Name: {self.name}</p><p>Age: {self.age}</p>{self.children}"
    // )]
    // struct Person<'a> {
    //     name: &'a str,
    //     age: u8,
    //     children: &'a Renderable
    // }

    // Should be expanded to:
    //
    struct Info;

    struct Person<'a> {
        name: &'a str,
        age: u8,
        children: Children<'a>,
    }

    // Target derived impls

    impl Template for Info {
        fn render(&self, context: &mut RenderContext) -> Result<(), Error> {
            context.render_literal("<div>")?;

            context.render_template(Person {
                name: "Fred",
                age: 35,
                children: Children::new(&|c| {
                    c.render_literal("<p>Lorem ipsum...</p>")?;

                    Ok(())
                }),
            })?;

            context.render_literal("</div>")?;

            Ok(())
        }
    }

    impl Template for Person<'_> {
        fn render(&self, context: &mut RenderContext) -> Result<(), Error> {
            context.render_literal("<p>Name: ")?;
            context.render_expression(&self.name)?;
            context.render_literal("</p><p>Age: ")?;
            context.render_expression(&self.age)?;
            context.render_literal("</p>")?;
            context.render_expression(&self.children)?;

            Ok(())
        }
    }

    assert_eq!(
        Info.render_to_string(),
        Ok("<div><p>Name: Fred</p><p>Age: 35</p><p>Lorem ipsum...</p></div>".to_string())
    );
}
