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

    //// Enums and Structs

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