extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()

    // Hide issue types
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("int_type")
        .blocklist_item("size_type")
        .blocklist_item("char_type")
        .blocklist_item("FP_INT_TONEAREST")
        .blocklist_item("FP_INT_TONEARESTFROMZERO")
        .blocklist_item("FP_INT_TOWARDZERO")
        .blocklist_item("FP_INT_DOWNWARD")
        .blocklist_item("FP_INT_UPWARD")
        .blocklist_item("std_collate_string_type")
        .blocklist_item("std_collate_byname_string_type")
        .blocklist_item("std_numpunct_string_type")
        .blocklist_item("std_numpunct_byname_string_type")
        .blocklist_item("std_value")
        .blocklist_item("std_size_type")
        .blocklist_item("std_basic_ostream_sentry")
        .blocklist_item("std_basic_istream_sentry_traits_type")
        .blocklist_item("std_basic_istream_sentry___streambuf_type")
        .blocklist_item("std_basic_istream_sentry___istream_type")
        .blocklist_item("__gnu_cxx___min")
        .blocklist_item("__gnu_cxx___max")
        .blocklist_item("_Value")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
