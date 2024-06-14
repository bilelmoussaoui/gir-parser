use std::{collections::HashMap, path::PathBuf};

use gir_parser::{prelude::*, Repository};

fn read_gir_file(
    namespaces: &mut HashMap<String, Repository>,
    gir_file: &str,
) -> Result<(), gir_parser::ParserError> {
    let path = PathBuf::from("./gir-files").join(gir_file);
    if namespaces.contains_key(gir_file) {
        return Ok(());
    }
    let repo = Repository::from_path(path)?;
    for namespace in repo.namespace_includes() {
        if !namespaces.contains_key(&namespace.as_package()) {
            read_gir_file(namespaces, &namespace.as_package_file())?;
        }
    }
    namespaces.insert(gir_file.to_owned(), repo);
    Ok(())
}

fn main() {
    let paths = std::fs::read_dir("./gir-files").unwrap();
    let mut namespaces = HashMap::new();

    for path in paths {
        let path = path.unwrap().path();
        let gir_file = path.file_name().unwrap().to_str().unwrap();
        read_gir_file(&mut namespaces, gir_file).unwrap();
    }

    for (name, repo) in &namespaces {
        println!("Library: {}", repo.namespace().name());
        println!("GIR File: {}", name);
        println!("Version: {}", repo.namespace().version());
        println!(
            "Dependencies: {}",
            repo.namespace_includes()
                .iter()
                .map(|n| n.as_package())
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!("Classes: {}", repo.namespace().classes().len());

        println!("\n             ##############             \n");
    }
}
