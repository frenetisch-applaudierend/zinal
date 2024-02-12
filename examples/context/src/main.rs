use zinal::{Children, Ctx, Template};

#[derive(Template)]
#[template(content = "<Layout><Content>Hello, World!</Content></Layout>")]
struct Page {
    #[provide_context]
    name: Ctx<String>,
}

#[derive(Template)]
#[template(content = "<main>{{self.children}}</main>")]
struct Layout<'a> {
    children: Children<'a>,
}

#[derive(Template)]
#[template(content = "
    <div>{{self.children}}</div>
    <div>From context: {{self.name}}</div>
")]
struct Content<'a> {
    #[from_context]
    name: Ctx<String>,
    children: Children<'a>,
}

fn main() {
    let page = Page {
        name: Ctx::new("Example".to_string()),
    };

    let rendered = page
        .render_to_string()
        .expect("should render without error");

    println!("{}", rendered);
}
