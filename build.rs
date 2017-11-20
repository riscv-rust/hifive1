use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    // Copy openocd.cfg to output directory
    File::create(out.join("openocd.cfg"))
        .unwrap()
        .write_all(include_bytes!("openocd.cfg"))
        .unwrap();
    println!("cargo:rerun-if-changed=openocd.cfg");

    println!("cargo:rerun-if-changed=build.rs");
}
