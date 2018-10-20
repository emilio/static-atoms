extern crate cc;


fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/test.cc")
        .compile("libfoo.a");
}
