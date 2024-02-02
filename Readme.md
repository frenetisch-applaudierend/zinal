# zinal

![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

[Zinal](https://en.wikipedia.org/wiki/Zinal) is a village in Switzerland, located in the municipality of Anniviers in the canton of Valais.

It is also a HTML templating library for Rust programs, focussing on composability.

> [!NOTE]
> While functional, this library is still in an early stage. Bugs may occur, and documentation and error messages are lacking.

## Features

* Composable templates with an intuitive syntax similar to JSX
* Embed arbitrary rust expressions and statements in your templates
* Compile-time errors for missing or incorrect template arguments
* Templates are built into the binary

## Usage

Add zinal as a dependency in your Cargo.toml:

```toml
[dependencies]
zinal = "0.1"
```

Define a template in code:

```rust
#[derive(Template)]
#[template(content = "<div>Hello, {{self.name}}!</div>")]
struct Hello<'a> {
  name: &'a str
}

fn main() {
  let hello = Hello { name: "Zinal" };
  println!(hello.render_to_string().unwrap());
}

// Prints
// <div>Hello, Zinal!</div>
```

Or reference a template file:

```html
<!-- File: templates/hello.html -->
<div>Hello, {{self.name}}!</div>
```

```rust
#[derive(Template)]
#[template(path = "hello.html")]
struct Hello<'a> {
  name: &'a str
}

fn main() {
  let hello = Hello { name: "Zinal" };
  println!(hello.render_to_string().unwrap());
}

// Prints
// <div>Hello, Zinal!</div>
```

You can use arbitrary rust expressions in your templates:

```rust
#[derive(Template)]
#[template(content = "<div>2 + 2 = {{2 + 2}}; {{ "Hello".to_uppercase() }}")]
struct Example;

fn main() {
  println!(Example.render_to_string().unwrap());
}
```

For more examples see the [examples](./examples) folder. For more information about the template syntax see [the syntax reference](./documentation/Syntax.md).
