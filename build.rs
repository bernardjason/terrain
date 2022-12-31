extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    //let cargo_target_dir = format!("{}/../../..",env::var("OUT_DIR").unwrap());
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [
        "GL_NV_command_list", // additional extension we want to use
    ])
        .write_bindings(
            StructGenerator, // different generator
            &mut file_gl
        )
        .unwrap();

    //fs::copy("src/index.html", format!("{}/index.html", cargo_target_dir)).unwrap();
    //println!("cargo:rustc-flags=-C -sUSE_SDL=2");
}
