use std::{fmt::Error, marker::PhantomData};

use crate::{derive::RenderExpression, Children, Context, Escaper, Template};

struct DummyBuilder<T>(PhantomData<T>);

impl<T> DummyBuilder<T> {
    fn new() -> Self {
        Self(PhantomData)
    }

    #[allow(dead_code)]
    fn build(self, _ctx: &Context) -> T {
        panic!("Unsupported")
    }
}

#[test]
#[allow(clippy::write_literal)]
#[allow(unused_variables)]
#[allow(dead_code)]
fn target_example() {
    // Source:
    //
    // #[derive(Template)]
    // #[template(
    //    type = "html",
    //    content = "<div><Person name="Fred" age={{35}}><p>Lorem ipsum...</p></Person></div>"
    // )]
    // struct Info;
    //
    // #[derive(Template)]
    // #[template(
    //    type = "html",
    //    content = "<p>Name: {{self.name}}</p><p>Age: {{self.age}}</p>{{self.children}}"
    // )]
    // struct Person<'a> {
    //     name: &'a str,
    //     age: u8,
    //     children: &'a Renderable
    // }

    // Should be expanded to:
    //
    struct Info<'a> {
        dummy: &'a str,
    }

    struct Person<'a> {
        name: &'a str,
        age: u8,
        children: Children<'a>,
    }

    // Target derived impls

    struct PersonBuilder<'a>(PhantomData<Person<'a>>);

    impl<'a> PersonBuilder<'a> {
        fn new() -> Self {
            Self(PhantomData)
        }

        fn build(self, context: &'a Context) -> Person<'a> {
            Person {
                name: context.get_param::<String>().as_ref().unwrap(),
                age: 10,
                children: Children::new(&|w, e, c| {
                    write!(w, "{}", "<p>Lorem ipsum...</p>")?;
                    Ok(())
                }),
            }
        }
    }

    impl<'a> Template for Info<'a> {
        type Builder = DummyBuilder<Info<'a>>;

        fn render(
            self,
            writer: &mut dyn std::fmt::Write,
            escaper: &dyn Escaper,
            context: &Context,
        ) -> Result<(), Error> {
            write!(writer, "{}", "<div>")?;

            {
                let _children = Children::new(&|w, e, c| {
                    write!(w, "{}", "<p>Lorem ipsum...</p>")?;
                    Ok(())
                });
                let template = Person::builder()
                    // .name("Fred".into())
                    // .age(35)
                    // .children(children)
                    .build(context);
                Template::render(template, writer, escaper, context)?;
            }

            write!(writer, "{}", "</div>")?;

            Ok(())
        }

        fn builder() -> Self::Builder {
            DummyBuilder::new()
        }
    }

    impl<'a> Template for Person<'a> {
        type Builder = PersonBuilder<'a>;

        fn render(
            self,
            writer: &mut dyn std::fmt::Write,
            escaper: &dyn Escaper,
            context: &Context,
        ) -> Result<(), Error> {
            write!(writer, "{}", "<p>Name: ")?;
            RenderExpression::render(&self.name, writer, escaper, context)?;
            write!(writer, "{}", "</p><p>Age: ")?;
            RenderExpression::render(&self.age, writer, escaper, context)?;
            write!(writer, "{}", "</p>")?;
            RenderExpression::render(&self.children, writer, escaper, context)?;

            Ok(())
        }

        fn builder() -> Self::Builder {
            PersonBuilder::new()
        }
    }

    let dummy = String::from("dummy");
    assert_eq!(
        Info { dummy: &dummy }.render_to_string(),
        Ok("<div><p>Name: Fred</p><p>Age: 35</p><p>Lorem ipsum...</p></div>".to_string())
    );
}
