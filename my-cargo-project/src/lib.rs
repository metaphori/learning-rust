mod my_mod2;

pub mod my_mod {
    pub fn f() -> i32 {
        // return 77 + super::my_mod2::g(); // OK
        return 77 + crate::my_mod2::g(); // OK
        // return 77 + ::my_mod2::g(); // ERROR: could not find `my_mod2` in `{{root}}` (NB: no matter that my_mod2 is private)
        // return 77 + my_mod2::g(); // ERROR: use of undeclared type or module `my_mod2`
        // return 77 + self::g(); // OK: self denotes path to current module
    }

    pub fn g() -> i32 { return 0; }

    #[test]
    fn h(){
        assert!(self::g()==0);
    }
}