fn main() {
    #[cfg(windows)] {
        use std::env;
        use std::path::PathBuf;
        
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = PathBuf::new();
        path.push(dir);
        path.push("glfw");
        println!("cargo:warning={:?}", path);
        println!("cargo:rustc-link-search=native={}", path.to_string_lossy());
    }
}
