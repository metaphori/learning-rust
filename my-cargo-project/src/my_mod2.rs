use super::my_mod;

pub fn g() -> i32 { return 1; }

pub fn h() -> i32 { return 1 + my_mod::g(); }