use zinal::*;

#[derive(Template)]
#[template("{{self.unknown_prop}}")]
struct Example {
    prop: u8,
}

fn main() {
    println!(
        "{}",
        Example { prop: 8 }
            .render_to_string()
            .expect("Should not fail")
    );
}
