use pulldown_cmark::{html::push_html, Parser};
use std::{env, fs, io::ErrorKind, path::PathBuf};

const CONTENT_MARKER: &str = "<!--CONTENT-->";

fn main() {
    println!("cargo:rerun-if-changed=assets/");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR unset"));
    match fs::remove_dir_all(out_dir.join("assets")) {
        Ok(()) => (),
        Err(err) if err.kind() == ErrorKind::NotFound => (),
        Err(err) => panic!("Couldn't remove output static assets directory: {err:?}"),
    };

    // Compile all markdown files in assets/pages into HTML, placing them in
    // ${OUT_DIR}/assets/pages.
    let out_pages = out_dir.join("assets/pages");
    fs::create_dir_all(&out_pages).expect("Couldn't create output pages directory");
    let template = fs::read_to_string("assets/template.html").expect("Couldn't read page template");
    for entry in fs::read_dir("assets/pages").expect("Couldn't read input pages directory") {
        let entry = entry.expect("Couldn't read input pages directory");

        let markdown = fs::read_to_string(entry.path()).expect("Couldn't read input page");
        let parser = Parser::new(&markdown);
        let mut content = String::new();
        push_html(&mut content, parser);
        let content = template.replacen(CONTENT_MARKER, &content, 1);

        let out_path = out_pages.join(entry.file_name()).with_extension("html");
        fs::write(out_path, content).expect("Couldn't write output page");
    }

    // Copy all files from assets/static into ${OUT_DIR}/assets/static.
    let out_static = out_dir.join("assets/static");
    fs::create_dir_all(&out_static).expect("Couldn't create output static directory");
    for entry in fs::read_dir("assets/static").expect("Couldn't read input static asset directory")
    {
        let entry = entry.expect("Couldn't read input static asset directory");
        fs::copy(entry.path(), out_static.join(entry.file_name()))
            .expect("Couldn't copy static asset");
    }
}
