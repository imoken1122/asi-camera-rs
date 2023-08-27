
extern crate bindgen;
use std::path::PathBuf;
use std::env;

fn generate_binding(){
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
    generate_binding()
}
