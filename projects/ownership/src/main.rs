#![allow(unused)]
// Ownership, borrowing, and slices ensure memory safety in Rust programs at compile time!

// Reference rules:
// - At any given time, you can have either but not both of the following:
// One mutable reference or any number of immutable references
// - References must always be valid (no lieutenant dangle!)

// String literals are &str to pointing to its start in the binary (a slice)

fn main() {
    string_literal_example();
    string_move_example();
    string_clone_example();
    stack_clone_example();
    functions_and_ownership_example();
    ownership_passing_example();
    hot_potato_example();
    borrowed_potato_example();
    mutate_borrowed_potato_example();
    data_race_example();
    slice_example();
}

fn stack_literal_to_heap(stack_literal: &str) -> String {
    String::from(stack_literal)
}

fn string_literal_example() {
    let mut s: String = stack_literal_to_heap("Hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn string_move_example() {
    let s1 = String::from("Hello, world!");
    let s2 = s1; // a move, s1 is now an invalidated reference

    // The stack allocated String data is copied, not the buffer
    // i.e. The String's ptr, length, and capacity
    // picture the following data as static member data of string:
    let ptr: usize = 0x0013FFA69;
    let len: usize = 13;
    let capacity: usize = 13;
    // These static data are known to exist at compile time, and assigned at runtime! (I think) 🤔

    // The data is copied, but done so as th equivalent as a
    // (C++) std::string s2 = std::move(s1);
    // Therefore, this next statement is an error, and can be
    // caught by the static analyzer!

    // println!("{}", s1); Err

    println!("{}", s2);

    // Summary:
    // Rust will never automatically create "deep" copies of data, therefore any automatic
    // copying can be assumed to be inexpensive in terms of runtime perf.
}

fn string_clone_example() {
    let s1 = String::from("Hello, world!");
    let s2 = s1.clone();

    println!("{} {}", s1, s2);
}

fn stack_clone_example() {
    let x = 5;
    let y = x;

    println!("{} {}", x, y);

    // stack allocated, therefore is a cheap "deep" copy, and thus is constructed
    // with a copy of the data and not via a move
    // For later: Traits, specifically the copy trait, of which can't be implemented
    // for a type if a drop trait has also been implemented
    // As quick general rule that isn't absolute, simple scalars can be 'Copy' and
    // anything that requires allocation is not 'Copy'

    // Some type that are 'Copy':
    // All the integer types (like u32)
    // The Boolean type
    // The character type char
    // All the floating points (like f64)
    // Tuples, but only if their composition types are 'Copy'
}

fn takes_ownership_of_string(s: String) {}

fn makes_copy_of_copyable_int(x: u64) {}

fn functions_and_ownership_example() {
    let s = String::from("Hi");

    takes_ownership_of_string(s);

    // Using a moved object: Err
    //println!("{}", s);

    let x = 5;
    makes_copy_of_copyable_int(x);

    println!("{}", x);
}

fn gives_ownership() -> String {
    String::from("Hi")
}

fn takes_ownership_and_gives_back(s: String) -> String {
    s
}

fn ownership_passing_example() {
    let mut s1 = gives_ownership();
    s1 = takes_ownership_and_gives_back(s1);
    println!("{}", s1);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn hot_potato_example() {
    // String must be passed back via tuple explicitly due to not being borrowed
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The lenth of '{}' is {}", s2, len);
}

fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

fn borrowed_potato_example() {
    let s1: String = String::from("Hello");
    let len = calculate_length_borrow(&s1); //must explicitly provide ref
    println!("The lenth of '{}' is {}", s1, len);
}

fn mutate_string(s: &mut String) {
    s.push_str(", world!");
}

fn mutate_borrowed_potato_example() {
    let mut s = String::from("Hello");
    mutate_string(&mut s);
    println!("{}", s);
}

fn data_race_example() {
    // Data race occurs when:
    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There's no mechanism being used to synchronize access to the data.

    let mut s = String::from("Hello");
    let r1 = &mut s;
    // Borrowing mut ref more than once in same scope: Err
    // let r2 = &mut s;
    print!("{}", r1);

    //To go around:
    let mut s1 = String::from(", world!");
    {
        let r3: &mut String = &mut s1;
    }
    let r4: &mut String = &mut s1;
    println!("{}", r4);

    // Additionally due to the data race design cornerstone:
    // One cannot simultaenously have both a mutable ref and a non-mutable ref,
    // or vice-versa, at the same time.
    //
    // Multiple mutables are fine, as that is just read-read concurrency that
    // does not require synchronization.
}

// Returns byte index of end of first word, else 1 + the last index of s (len)
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn second_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    let mut start = 0;
    let mut seen_first_space = false;
    for (i, &item) in bytes.iter().enumerate() {
        if ((item == b' ') && (seen_first_space == false)) {
            start = i + 1;
            seen_first_space = true;
        } else if (((item == b' ') || (i == s.len() - 1)) && (seen_first_space == true)) {
            return &s[start..i + 1];
        }
    }
    return &s[..];
}

// Can operate on String and &str!
fn first_word_experienced(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn slice_example() {
    let mut s = String::from("Hello, world!");
    let word = first_word(&s);
    s.clear();

    // word still has the value 5 here, but there's no mores tring that we could
    // meaningfully use the value 5 with. word is now totally invalid!
    // The solution: string slices!

    let mut s = String::from("Hello, world!");
    let hello = &s[0..6];
    let world: &str = &s[7..13];

    // Err
    // s.clear();
    // s.clear() takes a mutable ref, but hello and world are inherently non mutable slices,
    // thus, compiler error!
    println!("{} {}", hello, world);

    //Slices returned
    let first = first_word_slice(&s);
    let second = second_word_slice(&s);
    println!("{} {}", first, second);
}
