// extern crate my_cargo_project;
use my_cargo_project::my_mod;
use another_cargo_project::my_mod1::a;

fn main() {
    println!("Hello, world! {} {}", my_mod::f(), a());
}
