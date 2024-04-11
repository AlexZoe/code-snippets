use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rustc-link-search=build");
    println!("cargo:rustc-link-lib=opaque_type");

    let bindings = bindgen::Builder::default()
        .header("opaque.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
