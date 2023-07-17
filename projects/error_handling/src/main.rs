//! panic!() macro unwinds stack and deallocates by default
//! [profile.release]
//! panic = abort will call crt __abort() immediately without cleaning
//! this results in smaller binaries
//! Using the ENV_VAR %RUST_BACKTRACE%=1 or %RUST_BACKTRACE%=full will print
//! stacktrace to stderr

//! Rust panics on buffer overread, unlike C/C++ which reduces an attack vector
#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    //non_recoverable_example();
    //recoverable_with_result_t();
    //non_recoverable_with_shortcut();
    recoverable_with_propagation();
    recoverable_with_propagation_shortcut();
    recoverable_with_propagation_shortcut2();
    recoverable_with_propagation_shortcut3();
}

fn non_recoverable_example() {
    panic!("deallocate/ and or__abort()")
}

fn recoverable_with_result_t() {
    let example: &str = "hello.txt";
    let f = File::open(example);
    let f = match f {
        Ok(file) => file,
        // Match guard with ref to limit error move
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create(example) {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Unable to create file: {:?}", e);
            }
        },
        Err(error) => {
            panic!(
                "There was a problem other than the file not existing: {:?}",
                error
            );
        }
    };
}

fn non_recoverable_with_shortcut() {
    let example: &str = "hello.txt";
    // This will panic! on err
    // let f = File::open(example).unwrap();
    // This will panic on err with msg:
    //let f = File::open(example).expect("FileNotFound");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let target: &str = "username.txt";
    let f = File::open(target);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // propagate
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), //propagate
    }
}

fn recoverable_with_propagation() {
    let u_or_err = read_username_from_file();
    match u_or_err {
        Ok(s) => println!("Username: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_2() -> Result<String, io::Error> {
    let s: &str = "username.txt";
    let mut f = File::open(s)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; //? error chaining inline
    Ok(s)
}

fn recoverable_with_propagation_shortcut() {
    let u_or_err = read_username_2();
    match u_or_err {
        Ok(s) => println!("Username: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn recoverable_with_propagation_shortcut2() -> Result<String, io::Error> {
    Ok(read_username_2()?)
}

fn recoverable_with_propagation_shortcut3() -> Result<String, io::Error> {
    let s = read_username_2()?;
    Ok(s)
}

// Summary, .expect and .unwrap are habdy to prototype and scaffold
// unwrap and expect are perfect for unittests

// It's perfectly acceptable to call expect and unwrap when it's guaranteed
// to not err:
// use std::net::IpAddr;
// let home: IpAddr = "127.0.0.1".parse().unwrap();

// It's advisable to panic when you know you can no longer guarantee something,
// assume something, a contract has been broken, or an invariant has been
// invalidated.
// Like:
// --Invalid values
// --Contradictory values,
// --Missing values
// Plus one or more of the following states:
// - The bad state is not something that's expected to happen occasionally
// - Your code after this point needs to rely on not being in this state
// - There's not a good way to encode this information in the types you use


