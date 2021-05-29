fn main() {
    cc::Build::new().file("cpp/wrapper.cc").flag("-Wno-deprecated").flag("-Wno-deprecated-copy").compile("foo");
}
