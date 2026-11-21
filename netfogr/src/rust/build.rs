use std::path::Path;
use std::process::Command;

fn generate_r_import_lib(r_home: &str, out_dir: &str) {
    let r_dll = format!("{}/bin/x64/R.dll", r_home);
    let def_path = format!("{}/R.def", out_dir);
    let lib_path = format!("{}/libR.dll.a", out_dir);

    if Path::new(&lib_path).exists() {
        return;
    }

    if !Path::new(&r_dll).exists() {
        panic!("R.dll not found at {}", r_dll);
    }

    std::fs::create_dir_all(out_dir).expect("failed to create r_import_lib_gnu dir");

    let gendef_status = Command::new("gendef")
        .current_dir(out_dir)
        .arg(&r_dll)
        .status()
        .expect("failed to run gendef (needs MinGW-w64 tools on PATH)");

    if !gendef_status.success() {
        panic!("gendef failed for {}", r_dll);
    }

    let generated_def = format!("{}/R.def", out_dir);
    if generated_def != def_path {
        std::fs::rename(&generated_def, &def_path).expect("failed to move generated .def file");
    }

    let dlltool_status = Command::new("dlltool")
        .arg("-d")
        .arg(&def_path)
        .arg("-l")
        .arg(&lib_path)
        .arg("-D")
        .arg("R.dll")
        .status()
        .expect("failed to run dlltool (needs MinGW-w64 tools on PATH)");

    if !dlltool_status.success() {
        panic!("dlltool failed to build {}", lib_path);
    }

    return;
}

fn main() {
    let r_home = std::env::var("R_HOME").expect("R_HOME must be set to build netfogr");
    let include_dir = format!("{}/include", r_home);
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set by cargo");
    let target = std::env::var("TARGET").unwrap_or_default();

    cc::Build::new()
        .cpp(true)
        .file("entrypoint.cpp")
        .include(include_dir)
        .compile("netfogr_entrypoint");

    if target.contains("windows-gnu") {
        let import_lib_dir = format!("{}/r_import_lib_gnu", manifest_dir);
        generate_r_import_lib(&r_home, &import_lib_dir);
        println!("cargo:rustc-link-search=native={}", import_lib_dir);
    }

    if target.contains("msvc") {
        println!("cargo:rustc-link-arg=/EXPORT:R_init_netfogr");
    }

    return;
}
