use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    if !Path::new("binaryen/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status()
            .expect("error updating submodules");
    }

    let binaryen_cmake = cmake::Config::new("binaryen")
        .define("BUILD_STATIC_LIB", "ON")
        .define("ENABLE_WERROR", "OFF")
        .define("BUILD_TESTS", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/build/lib", binaryen_cmake.display());
    println!("cargo:rustc-link-lib=static=binaryen");
    print_deps(&binaryen_cmake);

    if let Some(cpp_stdlib) = get_cpp_stdlib() {
        println!("cargo:rustc-link-lib={}", cpp_stdlib);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}

// See https://github.com/alexcrichton/gcc-rs/blob/88ac58e25/src/lib.rs#L1197
fn get_cpp_stdlib() -> Option<String> {
    std::env::var("TARGET").ok().and_then(|target| {
        if target.contains("msvc") {
            None
        } else if target.contains("darwin") {
            Some("c++".to_string())
        } else if target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("musl") {
            Some("static=stdc++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    })
}

// See https://github.com/brson/miri/blob/master/build.rs#L70-L79
fn print_deps(path: &Path) {
    for e in path.read_dir().unwrap().filter_map(|e| e.ok()) {
        let file_type = e.file_type().unwrap();
        if file_type.is_dir() {
            print_deps(&e.path());
        } else {
            println!("cargo:rerun-if-changed={}", e.path().display());
        }
    }
}
