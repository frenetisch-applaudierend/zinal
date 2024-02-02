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

## Installation

Add zinal as a dependency in your Cargo.toml:

```toml
[dependencies]
zinal = "0.1"
```

## Examples

The following example shows some features of Zinal, like composing templates and embedding a rust for loop.

```rust
use zinal::*;

#[derive(Template)]
#[template(content = "
  <div>We greet the following people:</div>
  <ul>
  <#for name in &self.names #>
    <Person name={{name}} />
  <#end>
  </ul>
")]
struct Greetings {
  names: Vec<String>
}

#[derive(Template)]
#[template(content = "<li><p>{{self.name}}</p></li>")]
struct Person<'a> {
  name: &'a str,
}

let greetings = Greetings {
  names: vec!["Mary".to_owned(), "John".to_owned(), "Kate".to_owned(), "Agnes".to_owned()]
};

fn main() {
  // Prints (possibly with some insignificant whitespace differences):
  // <div>We greet the following people:</div>
  // <ul>
  // <li><p>Mary</p></li>
  // <li><p>John</p></li>
  // <li><p>Kate</p></li>
  // <li><p>Agnes</p></li>
  // </ul>
  println!("{}", greetings.render_to_string().unwrap()); 
}

```

You can either define a template directly in code...

```rust
#[derive(Template)]
#[template(content = "<div>Hello, {{self.name}}!</div>")]
struct Hello<'a> {
  name: &'a str
}
```

...or reference a template file. Template files must be in a top level folder called `templates`
but can be arbitrarily nested within.

```rust
#[derive(Template)]
#[template(path = "examples/hello.html")]
struct Hello<'a> {
  name: &'a str
}
```

```html
<!-- File: templates/examples/hello.html -->
<div>Hello, {{self.name}}!</div>
```

You can use arbitrary rust expressions in your templates...

```rust
#[derive(Template)]
#[template(content = "<div>2 + 2 = {{2 + 2}}; {{ "Hello".to_uppercase() }}")]
struct Example;
```

...as well as embed statements that get executed when the template renders.

```rust
#[derive(Template)]
#[template(content = "
  <div>Hello, World!</div>
  <# println!("Rendering Example template...") #>
")]
struct Example;
```

For more examples see the [examples](./examples) folder. For more information about the template syntax see [the syntax reference](./documentation/Syntax.md).
