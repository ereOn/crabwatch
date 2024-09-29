fn main() {
    pkg_config::Config::new().probe("libgpiod").unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
