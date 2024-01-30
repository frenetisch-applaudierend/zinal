use stardust::{Children, Renderable, Template};

#[derive(Template)]
#[template("<div><Person name='Fred' age={2}><p>Lorem ipsum...</p></Person></div>")]
struct Info;

#[derive(Template)]
#[template("<p>Name: {self.name}</p><p>Age: {self.age}</p>{self.children}")]
struct Person<'a, T: Renderable> {
    name: &'a str,
    age: T,
    children: Children<'a>,
}

fn main() {
    match Info.render_to_string() {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("{}", err),
    }
}
