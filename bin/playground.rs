use gir_parser::Repository;

fn main() {
    let paths = std::fs::read_dir("./gir-files").unwrap();
    let mut namespaces = std::collections::HashMap::new();
    let mut total_namespaces = 0;

    for path in paths {
        let path = path.unwrap().path();
        let gir_file = path.file_name().unwrap().to_str().unwrap();

        Repository::from_path_follow_namespaces_and_cache(&mut namespaces, gir_file, "./gir-files")
            .unwrap();
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
        total_namespaces += 1;
    }
    println!("Total namespaces read in: {total_namespaces}");
    println!("\n             *****************             \n");
}
