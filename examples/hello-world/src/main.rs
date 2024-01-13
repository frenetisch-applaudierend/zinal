use stardust::{content_type::Html, Children, RenderContext, Template};

#[derive(Template)]
#[template(
    type = "html",
    content = "<div><Person name='Fred' age={35}><p>Lorem ipsum...</p></Person></div>"
)]
struct Info;

#[derive(Template)]
#[template(
    type = "html",
    content = "<p>Name: {self.name}</p><p>Age: {self.age}</p>{self.children}"
)]
struct Person<'a> {
    name: &'a str,
    age: u8,
    children: Children<Html>,
}

struct Test {
    name: String,
}

impl Template<Html> for Test {
    fn render(&self, ctx: &mut RenderContext<'_, Html>) -> Result<(), std::fmt::Error> {
        ctx.render_template(Person {
            name: &self.name,
            age: 10,
            children: Children::new(|ctx| {
                ctx.render_literal("Hello")?;
                ctx.render_expression(&self.name)?;

                Ok(())
            }),
        })?;

        todo!()
    }
}

fn main() {
    match Info.render_to_string() {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("{}", err),
    }
}
