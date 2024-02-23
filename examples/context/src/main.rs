use zinal::{Children, Ctx, Template};

#[derive(Template)]
#[template(content = "<Layout><Content>Hello, World!</Content></Layout>")]
struct Page {
    #[provide_context]
    name: String,
}

#[derive(Template)]
#[template(content = "<main>{{@children}}</main>")]
struct Layout;

#[derive(Template)]
#[template(content = "
    <div>{{@children}}</div>
    <div>From context: {{self.name}}</div>
")]
struct Content<'a> {
    #[from_context]
    name: &'a String,
}

fn main() {
    let page = Page {
        name: "Example".to_string(),
    };

    let rendered = page
        .render_to_string()
        .expect("should render without error");

    println!("{}", rendered);
}
