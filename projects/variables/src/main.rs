#![allow(unused)]
#[macro_use]
extern crate static_assertions;

use std::num::Wrapping;

fn main() {
    basic_mutability();
    basic_shadowing();
    basic_static_typing();
    basic_scalar_types();
    basic_compound_types();
    basic_element_access();
}

fn basic_mutability() {
    //Err
    //let x = 5;
    //x = 6;

    //Ok
    let mut x = 5;
    x = 6;

    //Mutating a mutable variable's type: Err
    //x = "";
}

fn basic_shadowing() {
    //Using let consecutively shadows x and
    //retains its immutability after:

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    //Err
    //x = 6;

    //Mutating a shadowed variable's type: Ok
    let x = "";
}

fn basic_static_typing() {
    //Not enough information to infer type at compile time: Err
    //let guess = "42".parse().expect("Not a number!");

    //With type annotation: Ok
    let guess: u32 = "42".parse().expect("Not a number!");
}

fn basic_integer_types() {
    
    //integers
    //Overflow: Err
    //let x: i8 = 128;

    //Wrapping Add (Overflow Bypass): Ok
    const X: i8 = 0i8.wrapping_add((1 << 7) / 2).wrapping_add((1 << 7) / 2);
    const_assert!(X == i8::MIN);

    const Y: i16 = 0i16.wrapping_add((1 << 15) / 2).wrapping_add((1 << 15) / 2);
    const_assert!(Y == i16::MIN);

    const Z: i32 = 0i32.wrapping_add((1 << 31) / 2).wrapping_add((1 << 31) / 2);
    const_assert!(Z == i32::MIN);

    const A: i64 = 0i64.wrapping_add((1 << 63) / 2).wrapping_add((1 << 63) / 2);
    const_assert!(A == i64::MIN);

    const B: u8 = 0u8.wrapping_add(1 << 7).wrapping_add(1 << 7);
    const_assert!(B == u8::MIN);

    const C: u16 = 0u16.wrapping_add(1 << 15).wrapping_add(1 << 15);
    const_assert!(C == u16::MIN);

    const D: u32 = 0u32.wrapping_add(1 << 31).wrapping_add(1 << 31);
    const_assert!(D == u32::MIN);

    const E: u64 = 0u64.wrapping_add(1 << 63).wrapping_add(1 << 63);
    const_assert!(E == u64::MIN);

    const F: isize = 0isize.wrapping_add(isize::MAX).wrapping_add(1);
    const_assert!(F == isize::MIN);

    const G: usize = 0usize.wrapping_add(usize::MAX).wrapping_add(1);
    const_assert!(G == usize::MIN);
}

fn basic_number_literals() {
    const DECIMAL: i32 = 98_222;
    const HEX: u8 = 0xFF;
    const OCTAL: i8 = 0o77;
    const BINARY: u8 = 0b1111_0000;
    const BYTE: u8 = b'A';
}

fn basic_floating_point_types() {
    let x = 2.0; //f64
    let y: f32 = 3.0; // f32
}

fn basic_numeric_operations() {
    let sum = 5 + 10;

    let diff = 95.5 - 4.3;

    let prod = 4 * 30;

    let quot = 56.7 / 32.2;

    let rem = 43 % 5;
}

fn basic_boolean_type() {
    let t = true;

    let f: bool = false;
}

fn basic_character_type() {
    let c = 'z';
    let z = 'Ƶ';
    let emoji = '😎';
}

fn basic_scalar_types() {
    basic_integer_types();
    basic_number_literals();
    basic_floating_point_types();
    basic_numeric_operations();
    basic_boolean_type();
}

fn basic_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    //use named values as the result of pattern matching the compound type (destructuring):
    //println!("The value of y is: {}", y);

    //direct element access
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
}

fn basic_array() {
    //an array's element types are homogenous
    //an array's data is stack allocated
    //an array's size is immutable

    let a = [1, 2, 3, 4, 5];
}

fn basic_compound_types() {
    basic_tuple();
    basic_array();
}

#[allow(unconditional_panic)]
fn basic_element_access() {
    let coolness = ["not epic", "very epic", "gamer"];

    for level in coolness.iter() {
        println!("{}", level);
    }

    let mut index  = 0;
    while index < coolness.len() {
        println!("{}", coolness[index]);
        index += 1;
    }

    for (index, level) in coolness.iter().enumerate() {
        println!("{}: {}", index, level);
    }

    let mut index2 = 0;
    loop {
        println!("{}", coolness[index2]);
        index2 += 1;
        if index2 >= coolness.len() {
            break;
        }
    }

    //out of bounds: Panic
    //let x = coolness[3];
}