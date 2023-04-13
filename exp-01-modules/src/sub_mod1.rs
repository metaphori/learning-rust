pub fn f() {
    println!("sub_mod1");
    sub_sub_mod11::g();
}

pub mod sub_sub_mod11 {
    pub fn f() {
        println!("sub_sub_mod11");
        super::f();
    }

    pub(super) fn g() {
        println!("g");
    }
}