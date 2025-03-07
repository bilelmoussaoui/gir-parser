# gir-parser

[![docs](https://docs.rs/gir-parser/badge.svg)](https://docs.rs/gir-parser/) [![crates.io](https://img.shields.io/crates/v/gir-parser)](https://crates.io/crates/gir-parser) ![CI](https://github.com/bilelmoussaoui/gir-parser/workflows/CI/badge.svg)

A [GObject introspection](https://gi.readthedocs.io/en/latest/) [GIR files](https://gi.readthedocs.io/en/latest/tools/g-ir-generate.html) parser.

```rust,no_run
use gir_parser::{Repository, prelude::*};

let repository = Repository::from_path(format!("./gir-files/Gtk-4.0.gir")).unwrap();
for class in repository.namespace().classes().iter() {
    for method in class.methods() {
        println!("{}", method.name());
        println!("{}", method.doc().unwrap().text());
    }
}
```
