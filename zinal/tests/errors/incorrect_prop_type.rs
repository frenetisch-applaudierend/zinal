use zinal::*;

#[derive(Template)]
#[template("<Inner prop={{self.prop}} />")]
struct Outer {
    prop: u8,
}

#[derive(Template)]
#[template("{{self.prop}}")]
struct Inner {
    prop: String,
}

fn main() {
    println!(
        "{}",
        Example { prop: 8 }
            .render_to_string()
            .expect("Should not fail")
    );
}
