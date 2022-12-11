use std::{env, path::PathBuf};

const BLURAY_HEADER: &str = "#include <libbluray/bluray.h>\n
#include <libbluray/filesystem.h>\n 
#include <libbluray/keys.h>\n 
#include <libbluray/log_control.h>\n
#include <libbluray/meta_data.h>\n
#include <libbluray/overlay.h>\n
#include <libbluray/player_settings.h>\n";
const AACS_HEADER: &str = "#include <libaacs/aacs.h>\n";
const BDPLUS_HEADER: &str = "#include <libbdplus/bdplus.h>\n";

fn main() {
    let mut header = String::new();

    header.push_str(BLURAY_HEADER);
    println!("cargo:rustc-link-lib=bluray");

    #[cfg(feature = "aacs")]
    {
        header.push_str(AACS_HEADER);
        println!("cargo:rustc-link-lib=aacs");
    }

    #[cfg(feature = "bdplus")]
    {
        header.push_str(BDPLUS_HEADER);
        println!("cargo:rustc-link-lib=bdplus");
    }

    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", header.as_str())
        // Fix error E0133 (see https://github.com/rust-lang/rust/issues/46043)
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
