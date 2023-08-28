
extern crate bindgen;
use std::path::PathBuf;
use std::env;

fn generate_binding(){

    let path = std::fs::canonicalize("./libasi/vendor/mac");
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}",path.unwrap().display());
    //println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libpath);

    println!("cargo:rustc-env=DYLD_LIBRARY_PATH=./libasi/vendor/mac");
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=ASICamera2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=./libasi/include/ASICamera2.h");
    let bindings = bindgen::Builder::default()
            .header("./libasi/include/ASICamera2.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}
fn main(){
    //TODO
    //  support each OS
    //  if already exist bindings.rs, to skip generate_bindng()
    generate_binding()
}
