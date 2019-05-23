mod my_mod2;

pub mod my_mod {
    pub fn f() -> i32 {
        return 77 + super::my_mod2::g(); // OK
        // return 77 + ::my_mod2::g(); // ERROR: could not find `my_mod2` in `{{root}}`
        // return 77 + my_mod2::g(); // ERROR: use of undeclared type or module `my_mod2`
    }
}