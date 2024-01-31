use stardust::{Children, Template};

#[derive(Template)]
#[template("<div><Person name='Fred'><p>Lorem ipsum...</p></Person></div>")]
pub struct Info;

#[derive(Template)]
#[template("<p>Name: {self.name}</p><p>Minor: {self.minor}</p>{self.children}")]
struct Person<'a> {
    name: &'a str,
    #[optional(default = true)]
    minor: bool,
    children: Children<'a>,
}

fn main() {
    match Info.render_to_string() {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("{}", err),
    }
}
