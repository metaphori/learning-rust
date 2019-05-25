// extern crate my_cargo_project;
use my_cargo_project::my_mod;
use another_cargo_project::my_mod1::a;

static mut STASH: &i32 = &0; // statics must be initialised

fn main() {
    println!("Hello, world! {} {}", my_mod::f(), a());

    //// Immutable and mutable variables

    let (v1,v2) = (7.8, true); // IMMUTABILITY by default; also note INFERENCE
    let v1 = v1+1.; // Shadowing
    // v1 = v1 +1; // Error

    let v: Vec<i32> = Vec::new();
    println!("{}", v.len());
    // v.push(1); // Error: cannot borrow as mutable

    let mut v: Vec<i32> = Vec::new();
    v.push(1); // Ok
    println!("{}", v.len());

    // let i: i32 = {  10; }; // ERROR: expected i32, found ()
    let i: i32 = { fn f(){}; 20; ; 10 }; // OK (local fun; 20 is dropped; empty statement; return 10)

    //// Scalar types
    let i: i64 = -20000;
    let j: u8 = 255;
    let x: f32 = std::f32::INFINITY;
    let c: char = 'c';
    let b: bool = true;
    let u: () = { println!("hello") };

    // let v = (1 as i32) + (2 as i64); // ERROR

    //// Basic I/O
    println!("{:.4} {} {} {:b} {:?}", std::f64::consts::PI, true, "foo", 7, Some(10));
    // 3.1416 true foo 111 Some(10)

    let mut s: String = String::new();
    if let Ok(l) = std::io::stdin().read_line(&mut s) {
        let i = s[..l-1].parse::<i32>().expect(&format!("Invalid input {}", s));
        println!("READ INT: {}", i);
    }

    //// COMPOUND TYPES
    // Tuples
    let (v1,v2): (i32,bool) = (77, true);        // Destructuring
    let tp: (i32,bool) = (77, true);
    println!("{}", tp.0 as f64);

    // Arrays
    let a1: [i32;5] = [1, 2, 3, 4, 5]; // Type include size
    //a1[500]; // compile-time error
    let last = a1[a1.len()-1];
    let aslice: &[i32] = &a1[1..];
    let arr = [1,2,3];

    // Vectors
    let mut v1: Vec<i32> = Vec::new();
    v1.push(10);
    let v2 = vec![1.0, 2.0, 3.0];
    println!("{:?} {:?}", v1, v2);

    // String
    let s0: &'static str = "foo";
    let s1: String = "foo".to_string();
    let s2 = String::from("bar");
    let s3: &str = &s2[..s2.len()-1];
    println!("{} {} {} {}", s0, s1, s2, s3);

    // Box
    let v: (i32,&str) = (12, "eggs");
    let b = Box::new(v); // allocate the tuple on the heap
    println!("{}", b.1);

    //// Enums and Structs
    struct S { x: f32, y: f32 }   // Named-field struct
    struct T(i32,char);           // Tuple-like struct
    struct U {}                  // Unit-like struct
    enum E { A, B(u32) }         // Enum (sum type)

    let s1 = S { x: 1.2, y: 5. };
    let t1 = T(77,'a');
    let u: U = U{};
    let e = E::B(5) + E::B(2).inc();

    use E::B;

    impl E {
        fn inc(&self) -> Self { match self { B(i) => B(i+1), _ => E::A }  }
    }

    impl Add for E {
        type Output = E;

        fn add(self, rhs: E) -> Self::Output {
            match (self,rhs) { (B(i),B(j)) => B(i+j), _ => E::A }
        }
    }

    println!("E::B => {}", match e.inc() { B(i) => i, _ => 0 });

    let r: Result<i32,&str> = Ok(10);


    #[cfg(debug)] enum MyOption<T> { // Defined in Rust stdlib
        None, Some(T)
    };
    let o1 = Some(10);
    println!("{:?}", o1);

    struct MyComplex<T> { // Named-field struct
        re: T, im: T
    };
    let mut z = MyComplex::<f64> { re: 8.0, im: 4.0 };

    struct Tp(i32,char); // Tuple-like struct
    let t1 = Tp(77,'a');

    println!("Named-field struct val: {:?}\nTuple-like struct val: {:?}", z.re, t1.0);

    //// On errors

    type RI = Result<i32,String>;
    fn ferr(r1: RI, r2: RI) -> RI { return Ok(r1?+r2?); }
    let ri: RI = ferr(Ok(60),Ok(30));
    println!("RI: {:?}", ri.unwrap());
    // let v: Option<i32> = ri.ok(); // ERROR: value used here after move

    println!("Panic recovery: {:?}", std::panic::catch_unwind(|| {
        let y = 0; let x = 1 / y; x
    }).ok());

    //// Lambdas and closures

    let mut z = 0;
    let f2: fn(i32,i32)->i32 = |x,y| { x+y };
    let f3 = |x:i32,y:i32| {x+y}; // partial typing
    let f: &Fn(i32,i32)->i32 = &|x,y| { z+x+y };
    println!("Closure call: {} {}", f(10,10), z);

    let mut z = 0;
    let mut f = |x,y| { z=z+1; x+y };
    println!("Mutable closure call: {} {}", f(10,20), z);

    //// Arrays, vectors, slices

    let a: [f64; 4] =     [0.0, 0.707, 1.0, 0.707];
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];

    let sa: &[f64] = &a; // automatic conversion: array ref -> slice ref
    let sv: &[f64] = &v; // automatic conversion: vector ref -> slice ref
    let sv2: &[f64] = &v[..];

    fn print(s: &[f64]){ for e in s { print!("{}; ",e) } println!(""); }
    print(sv); print(sv2);

    //// Error handling: results

    fn rf(i: i32) -> Result<i32,String> { if i>=0 { Ok(i) } else { Err(String::from("err")) } }
    let mut wres = rf(33);
    let isOk: bool = wres.is_ok();
    let ok1: Option<&i32> = wres.as_ref().ok(); // borrow of 'wres' occurs here
    //let ok: Option<i32> = wres.ok(); // ERROR: cannot move out of 'wres' because it is borrowed
    let ok1 = wres.unwrap_or(88);
    println!("{:?}", ok1);

    //// Ownership, moves

    fn vgen() -> Vec<i32> {
        let v = vec![1,2,3];
        return v; // ownership moved from local var 'v' to the caller
    }
    let v = vgen();
    println!("{:?}", v);

    let mut v = vec!["bob".to_string(), "mark".to_string()];
    // let x = v[0]; // ERROR: cannot move out of indexed content (note: ok with int vals)
    // Other operations do support moving elements out
    let y = v.pop().unwrap(); // pop value off the end of the vector (ok)

    for mut s in v { // for loop takes ownership of the vector
        s.push('!');
    }
    // println!("{:?}",v); // ERROR: value 'v' used after move

    //// Copy

    let mut a = 0;
    {
        let b = 7;
        a = b; // scalars are Copy
        println!("b = {}", b);
    }

    struct St1 { x: i32};
    // #[derive(Copy)] struct St2 { s1: St1 }; // ERROR: field 's1' does not implement Copy

    fn fs(s: String) { println!("fs: {}", s); }
    fn fi(i: i32){ println!("fi: {}", i); }
    fn give_s() -> String { let s = String::from("!"); s } // Ownership moved, nothing dealloc'ed
    fn chg_s(mut s: String) -> String { s.push_str("!"); s } // Borrows mutably

    let mut s = "hello".to_string();
    fs(s); // moves ownership of 's' to the function!
    // println!("s: {}", s); // Cannot reference 's'

    let i = 10;
    fi(i); // i gets copied
    println!("i: {}", i); // i can be used here

    let mut s2 = give_s(); // give_s() moves its return value to s2
    s2 = chg_s(s2); // takes ownership and returns it

    //// Lifetime

    // fn f(p: &i32) { // Actually a shortcut for:  fn f<'a>(p: &'a i32)
    //    unsafe { STASH = p; } // COMPILE-TIME ERROR: lifetime `'static` required
    // }
    fn f(p: &'static i32) {
        unsafe { STASH = p; }
    }

    struct P<'a> { r: &'a i32 };
    let p: P;
    {
        let x = 10;
        // p = P { r: &x }; // ERROR: borrowed value does not live long enough
    }
    // assert_eq!(*p.r, 10); // otherwise, this would be bad!

    struct S1<'a> {
        x: &'a i32,
        y: &'a i32
    }
    struct S2<'a, 'b> {
        x: &'a i32,
        y: &'b i32
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S2 { x: &x, y: &y }; // s' lifetime must not outlive x's and y's
            r = s.x; // s' lifetime must enclose r's
        }
    } // ERROR: (with S1) y does not live long enough
    println!("{:?}", r);

    //// Sharing vs. mutation

    fn extend(v: &mut Vec<f64>, s: &[f64]) { for e in s { v.push(*e); } }

    let mut v1 = Vec::new(); let v2 = vec![0.0, 1.0, 0.0, -1.0];
    extend(&mut v1, &v2);
    //extend(&mut v1, &v1); // ERR: can't borrow v1 as immutable cause is also borrowed as mutable

    //// Operator overloading

    use std::ops::*; // imports Add and AddAsign
    #[derive(Clone,Copy,Debug)] struct Complex<T> { re: T, im: T }
//    impl<T> Add for Complex<T> where T: Add<Output=T> {
//        type Output = Self;
//        fn add(self, rhs: Self) -> Self { Complex{ re:self.re+rhs.re, im:self.im+rhs.im }}
//    } // notice that operands are taken by value

    // ERROR: impl doesn't use types inside crate
//    impl Add<i64> for i32 {
//        type Output = i64;
//
//        fn add(self, rhs: i64) -> Self::Output {
//            self as i32 + rhs
//        }
//    }

    // We may loose constraints to allow diff types for + and diff result type
    impl<L, R, O> Add<Complex<R>> for Complex<L> where L: Add<R, Output=O> {
        type Output = Complex<O>;
        fn add(self, rhs: Complex<R>) -> Self::Output { Complex { re: self.re+rhs.re , im: self.im+rhs.im }}
    } // However, may not be much more useful that the simpler generic def

    impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
        fn add_assign(&mut self, rhs: Complex<T>) { self.re+=rhs.re; self.im+=rhs.im; }
    } // for c1 += c2

    let c1 = Complex::<i64> { re: 10, im: 20 };
    let c2 = Complex::<i32> { re: 10, im: 20 };
    let c3: Complex<i64> = c1 + c1;
    println!("Complex: {:?}", c3);

    //// More

    let my_str = "hello".to_string();
    let fdrop1 = || drop(my_str);
    // let fdrop2 = || drop(my_str); // ERROR: var `my_str` moved due to use in closure
    fdrop1(); // ok
}

#[test]
fn another_test(){
    println!("Also the 'main', executable crate may have its own tests");
    assert!(1 > 0);
}