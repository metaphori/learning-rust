use std::{
    fs::File,
    io,
    io::Read,
};

/*
    If there is no 'impl' keyword we get:
        * error[E0782]: trait objects must include the `dyn` keyword
    If we substitute 'impl' with 'dyn' we get:
        * Mismatches type (on Ok(f)): expected `Result<dyn Read, Error>`, found `Result<File, _>`
        * error[E0277]: the size for values of type `(dyn std::io::Read + 'static)` cannot be known at compilation time
        ...
    Let's see why we need that 'impl':
        pub enum Result<T, E> { Ok(T), Err(E) }
        impl<T, E> Result<T, E>
        pub trait Read { 
            fn bytes(self) -> Bytes<Self>
            // ...
        }
        impl Read for File
        impl Read for Stdin
        // ...
        pub struct File
        impl File {
            pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File>
            // ...
        }
}
    
*/
fn returning_an_impl_trait_wrapped_inside_result() -> io::Result<impl io::Read> {
    let f = File::open("src/main.rs")?;
    Ok(f)
}
fn returning_an_impl_trait_wrapped_inside_result_alternative() -> io::Result<Box<dyn io::Read>> { // NB: dyn Trait must be boxed
    let f = File::open("src/main.rs")?;
    Ok(Box::new(f))
}


fn f(x: String) -> String { x + "!" }
fn g(x: &str) -> String { String::from(x) + "!" }

fn ownership_simple() {
    let x = String::from("foo");
    f(x); // x is moved into f
    // println!("{}", x); // cannot borrow value here (after move in previous line)
    // println!("{}", &x); // same error

    let x = String::from("bar");
    g(&x); // just borrow 
    println!("{}", x); // ok   

    let x = String::from("foobar");
    f(x.clone()); // x is cloned, avoiding move of the original
    println!("{}", x); // ok    
}

fn main() {
    // *** IMPL TRAIT *** <https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html> <https://varkor.github.io/blog/2018/07/03/existential-types-in-rust.html
    let result = returning_an_impl_trait_wrapped_inside_result();
    // get string of io::Read and print it
    println!("{:?}", result.unwrap().bytes().map(|b| b.unwrap() as char).collect::<String>());

    let another = returning_an_impl_trait_wrapped_inside_result_alternative();
    println!("{:?}", another.unwrap().bytes().map(|b| b.unwrap() as char).collect::<String>());

    ownership_simple();
}
