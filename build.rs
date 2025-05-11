fn main() {
    cc::Build::new()
        .file("sqlite-vec.c")
        .include(".")
        .define("SQLITE_CORE", None)
        .compile("sqlite_vec0");
}
