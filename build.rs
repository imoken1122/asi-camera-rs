
extern crate bindgen;
use std::path::PathBuf;
use std::env;
use std::fs;
pub fn get_libpath() -> String{
   let mut libpath = "";

    // only x86_64
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
    return libpath.to_string();
}

fn set_dylib(archpath:String){
    let libpath = "./libasi-build/vendor/".to_string() + &archpath;
    let path = std::fs::canonicalize(&libpath);
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}",path.unwrap().display());
    //println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libpath);

    println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}",libpath);
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}",libpath);
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=ASICamera2");

  
}

fn create_dir() {
    let directory_name = "output"; // 作成するディレクトリ名

    // ディレクトリが存在しない場合に作成
    if !fs::metadata(directory_name).is_ok() {
        match fs::create_dir(directory_name) {
            Ok(_) => println!("Created directory: {}", directory_name),
            Err(e) => eprintln!("Error creating directory: {}", e),
        }
    } else {
        println!("Directory already exists: {}", directory_name);
    }
}


fn main(){
    set_dylib(get_libpath());
    create_dir();
}
