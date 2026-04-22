extern crate pkg_config;
use pkg_config::find_library;

fn main() {
  // For both libxslt and libexslt we first ask pkg-config; if that fails
  // (e.g. minimal installs without the .pc files) we emit a plain
  // `cargo:rustc-link-lib=dylib=…` so systems with the library on the
  // default linker search path still link. libexslt provides the str:*,
  // math:*, set:*, date:* extension functions used by many stylesheets —
  // it is required so `exsltRegisterAll` is resolvable.
  if find_library("libxslt").is_err() {
    println!("cargo:rustc-link-lib=dylib=xslt");
  }
  if find_library("libexslt").is_err() {
    println!("cargo:rustc-link-lib=dylib=exslt");
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
