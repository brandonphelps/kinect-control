use cmake;


fn main() {

    let dst = cmake::build("libfreenect");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=freenect");
}
