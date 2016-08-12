extern crate cmake;

fn main() {
    let dst = cmake::build("wrapper");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=obs_plugin_wrapper");

    println!("cargo:rustc-link-lib=dylib=obs");
}
