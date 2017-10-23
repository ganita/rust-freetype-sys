extern crate cmake;
extern crate pkg_config;

use std::env;

fn main() {
    if env::var_os("FREETYPE_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::find_library("freetype").is_ok() {
            return
        }
    }

    let lib = env::var("CARGO_MANIFEST_LINKS").unwrap();
    assert_eq!(lib, "freetype");

    let mut cmd = cmake::Config::new("freetype");
    cmd.define("WITH_ZLIB", "OFF");
    cmd.define("WITH_BZip2", "OFF");
    cmd.define("WITH_PNG", "OFF");
    cmd.define("WITH_HarfBuzz", "OFF");

    let dest = cmd.build();
    let path = dest.display();
    
    println!("cargo:rustc-link-search=native={}/lib", path);
    println!("cargo:rustc-link-lib=static=freetyped");    

    println!("cargo:libs={}/lib", path);
    println!("cargo:include={}/include/freetype2", path);
}
