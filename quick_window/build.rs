fn main() {
    #[cfg(windows)] {
        println!("cargo:rustc-link-search=native=glfw");
    }
}
