use cc::Build;
use std::path::PathBuf;

fn main() {
    let md4c_fir = PathBuf::from("md4c");
    let md4c_src: Vec<PathBuf> = ["entity", "md4c_html", "md4c"]
        .iter()
        .map(|s| s.into())
        .collect();
    let bindings_path = PathBuf::from("src/md4c_sys");

    for src in &md4c_src {
        let header = md4c_fir.join(src.with_extension("h"));
        println!(
            "cargo:rerun-if-changed={}",
            header.to_string_lossy()
        );

        let bindings = bindgen::Builder::default()
            .header(header.to_string_lossy())
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(
                bindings_path.join(src).with_extension("rs"),
            )
            .expect("Failed to generate bindings");
    }

    let mut builder = Build::new();

    for src in &md4c_src {
        builder.file(md4c_fir.join(src.with_extension("c")));
    }

    builder.compile("md4c");
}
