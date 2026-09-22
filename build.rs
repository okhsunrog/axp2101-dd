fn main() {
    println!("cargo:rerun-if-changed=device.ddsl");
    println!("cargo:rustc-link-arg=-Tlinkall.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
}
