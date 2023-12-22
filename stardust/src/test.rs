use std::fmt::{Error, Write};

use crate::{Escaper, NoopEscaper, Renderable, Template};

#[test]
fn hello_world() {
    struct HelloWorld<'a> {
        name: &'a dyn Renderable,
    }

    impl Renderable for HelloWorld<'_> {
        fn render_to(&self, writer: &mut dyn Write, escaper: &dyn Escaper) -> Result<(), Error> {
            write!(writer, "Hello, ")?;
            self.name.render_to(writer, escaper)?;

            Ok(())
        }
    }

    impl Template for HelloWorld<'_> {}

    let name = "World!";
    let hello = HelloWorld { name: &name };

    assert_eq!(
        hello.render_to_string(&NoopEscaper),
        Ok("Hello, World!".to_string())
    );
}

#[test]
fn target_example() {
    // Template for Info:
    // <div><Person name="Fred" age={35}><p>Lorem ipsum...</p></Person></div>
    //
    // Template for Person:
    // <p>Name: {self.name}</p><p>Age: {self.age}</p>{self.children}

    struct Info;

    struct Person<'a> {
        name: &'a str,
        age: u8,
        children: &'a dyn Renderable,
    }

    // Target derived impls

    impl Template for Info {}
    impl Renderable for Info {
        fn render_to(&self, writer: &mut dyn Write, escaper: &dyn Escaper) -> Result<(), Error> {
            write!(writer, "<div>")?;

            {
                let __child = Person {
                    name: "Fred",
                    age: 35,
                    children: &"<p>Lorem ipsum...</p>",
                };
                __child.render_to(writer, escaper)?;
            }

            write!(writer, "</div>")?;

            Ok(())
        }
    }

    impl Template for Person<'_> {}
    impl Renderable for Person<'_> {
        fn render_to(&self, writer: &mut dyn Write, escaper: &dyn Escaper) -> Result<(), Error> {
            write!(writer, "<p>Name: ")?;

            self.name.render_to(writer, escaper)?;

            write!(writer, "</p><p>Age: ")?;

            self.age.render_to(writer, escaper)?;

            write!(writer, "</p>")?;

            self.children.render_to(writer, escaper)?;

            Ok(())
        }
    }

    assert_eq!(
        Info.render_to_string(&NoopEscaper),
        Ok("<div><p>Name: Fred</p><p>Age: 35</p><p>Lorem ipsum...</p></div>".to_string())
    );
}
