use cc;

fn main() {
    cc::Build::new()
        .file("../cc_code/main.c")
        .compile("hello");
    println!("cargo:rerun-if-changed=src/hello.c");
}
