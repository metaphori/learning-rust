// this is the crate's root module

mod sub_mod1; // declared module - code imported from src/sub_mod1.rs
mod sub_mod2; // declared module - code imported from src/sub_mod2/mod.rs

fn main() {
    println!("Hello, world!");
    ::std::println!("Hello, world! (via absolute path)");
    sub_mod1::f();
    sub_mod2::f();
    crate::sub_mod2::f(); // same as above line, but using an absolute path
    self::sub_mod2::f(); // same as above, since self=crate here
    use sub_mod1::sub_sub_mod11::f;
    sub_mod1::sub_sub_mod11::f();
    f(); // same as above line thanks to import via `use`
}
