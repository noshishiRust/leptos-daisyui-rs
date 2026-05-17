use std::fs;

fn main() {
    let doc_dir = std::path::Path::new("../doc/components");

    // Track the directory itself (new files added/removed)
    println!("cargo:rerun-if-changed={}", doc_dir.display());

    // Track each individual markdown file (content changes)
    if let Ok(entries) = fs::read_dir(doc_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}
