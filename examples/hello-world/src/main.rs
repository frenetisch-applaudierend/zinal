use stardust::{Renderable, Template};

#[derive(Template)]
#[template(
    type = "html",
    content = "<div><# println!(\"Injected\") #><Person name='Fred' age={35}><p>Lorem ipsum...</p></Person></div>"
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
    children: &'a dyn Renderable,
}

fn main() {
    match Info.render_to_string() {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("{}", err),
    }
}
