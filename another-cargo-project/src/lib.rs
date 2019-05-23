#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod my_mod1 {
  pub fn a() -> i32 { return 2; }
  fn b() -> i32 { return 3; }
}

mod my_mod2 {
  pub fn c() -> i32 { return 4; }
  pub fn d() -> i32 { return 5; }
}

