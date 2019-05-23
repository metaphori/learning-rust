use ::my_cargo_project::my_mod::g; // the prefix "::" is not needed

// Notice how different uses of 'use' (outside or inside a module) change paths to reference items

#[cfg(test)]
mod my_integration_tests {
    use ::another_cargo_project::my_mod1::a; // the prefix "::" is not needed

    #[test]
    fn integration_test1() {
        assert!(a() == 2);
        assert!(super::g() == 0);
    }
}