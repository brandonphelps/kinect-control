use cmake;

fn main() {

    let dst = cmake::build("libfreenect");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    // println!("cargo:rustc-link-search=natiusb-1
    println!("cargo:rustc-link-lib=static=freenect");
    println!("cargo:rustc-link-lib=static=usb-1.0");
    println!("cargo:rustc-link-lib=dylib=udev");

    println!("cargo:rerun-if-changed=wrapper.h");

    // let bindings = bindgen::Builder::default()
    //     .header("wrapper.h")
    //     .clang_arg("-I/libfreenect/include")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate()
    //     .expect("unable to generate bindings");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // bindings.write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings");
}
