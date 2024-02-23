use zinal::Template;

#[derive(Template)]
#[template("<div><Person name='Fred'><p>{{self.age}}Lorem ipsum...</p></Person></div>")]
pub struct Info {
    #[from_context]
    age: u8,
}

#[derive(Template)]
#[template("<p>Name: {{self.name}}</p><p>Minor: {{self.minor}}</p>{{@children}}")]
struct Person<'a> {
    name: &'a str,
    #[optional(default = true)]
    minor: bool,
}

fn main() {
    match (Info { age: 10 }).render_to_string() {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("{}", err),
    }
}
