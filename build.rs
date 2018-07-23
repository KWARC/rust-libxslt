extern crate pkg_config;
use pkg_config::find_library;

fn main() {
  if find_library("libxslt").is_ok() {
    return;
  } else {
    panic!("Could not find libxslt using pkg-config");
  }
  // // The bindgen::Builder is the main entry point
  // // to bindgen, and lets you build up options for
  // // the resulting bindings.
  // let bindings = bindgen::Builder::default()
  //       // The input header we would like to generate
  //       // bindings for.
  //       .header("wrapper.h")
  //       // Homebrew location of libxslt headers.
  //       .clang_arg("-I/usr/include/libxml2")
  //       .clang_arg("-I/usr/include/libxslt")
  //       // Finish the builder and generate the bindings.
  //       .generate()
  //       // Unwrap the Result and panic on failure.
  //       .expect("Unable to generate bindings");

  // // Write the bindings to the $OUT_DIR/bindings.rs file.
  // let out_path = PathBuf::from("src");
  // bindings
  //   .write_to_file(out_path.join("bindings.rs"))
  //   .expect("Couldn't write bindings!");
}
