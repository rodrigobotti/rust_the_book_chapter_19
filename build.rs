fn main() {
    cc::Build::new().file("src/c_module.c").compile("c_module");
}
