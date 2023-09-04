
extern crate bindgen;
use std::path::PathBuf;
use std::env;

fn generate_binding(archpath: &str){
    let libpath = "./vendor/".to_string() +  archpath;

    let path = std::fs::canonicalize(&libpath);
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}",path.unwrap().display());
    //println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libpath);

    println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}",&libpath);
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=ASICamera2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=./include/ASICamera2.h");
    let bindings = bindgen::Builder::default()
            .header("./include/ASICamera2.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}
fn main(){
    let mut libpath = "";

    // only intel x86_64 or Rosseta2 
    if cfg!(target_os = "macos") {
        libpath = "mac";
    };


    //The following operating systems have not been tested
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        libpath = "linux/x86";
    };

    if cfg!(target_os = "linux") && cfg!(target_arch = "x86") {
        libpath = "linux/x64";
    };

    if cfg!(target_os = "linux") && cfg!(target_arch = "arm") {
        libpath = "linux/arm7";
    };

    if cfg!(target_os = "linux") && cfg!(target_arch = "aarch64") {
        libpath = "linux/arm8";
    };

    if cfg!(target_os = "windows") {
        libpath = "windows/x86";
    };
    generate_binding(libpath);
}
