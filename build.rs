// Copyright 2017 Sreejith Krishnan R <sreejith@ganita.io>
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
// except in compliance with the License. You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions
// and limitations under the License.

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
