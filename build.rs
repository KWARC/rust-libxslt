extern crate pkg_config;
use pkg_config::find_library;

fn main() {
  // Re-run if the opt-in static toggle flips.
  println!("cargo:rerun-if-env-changed=LIBXSLT_STATIC");
  // Opt-in static link: with LIBXSLT_STATIC set and PKG_CONFIG_PATH pointing at a
  // non-system prefix carrying PIC `libxslt.a` + `libexslt.a`, `.statik(true)`
  // emits `static=exslt`/`static=xslt` (+ transitive libxml2 and `-lm` from the
  // .pc files). libexslt is probed FIRST so its archive precedes the xslt/xml2 it
  // depends on in the static link order. pkg-config's static guard refuses a
  // `static=` for /usr/lib system paths, so the non-system prefix is what makes
  // this work. Default (env unset) is the original dynamic behaviour below. Used
  // for the self-contained, SONAME-independent release binary.
  if std::env::var_os("LIBXSLT_STATIC").is_some() {
    let mut cfg = pkg_config::Config::new();
    cfg.statik(true);
    cfg
      .probe("libexslt")
      .expect("static libexslt via pkg-config (set PKG_CONFIG_PATH to the static prefix)");
    cfg
      .probe("libxslt")
      .expect("static libxslt via pkg-config (set PKG_CONFIG_PATH to the static prefix)");
    return;
  }

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
