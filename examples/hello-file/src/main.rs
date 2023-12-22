use stardust::{Renderable, Template};

#[derive(Template)]
#[template(path = "person.html")]
struct Person {
    name: String,
    age: u8,
    children: Vec<String>,
}

#[derive(Template)]
#[template(type = "html", content = "<p>Name: {self.name}</p>")]
struct Child<'a> {
    name: &'a str,
}

fn main() {
    let person = Person {
        name: "Homer".to_string(),
        age: 42,
        children: vec!["Bart".to_string(), "Lisa".to_string(), "Maggie".to_string()],
    };

    match person.render_to_string() {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("{}", err),
    }
}
