#![allow(unused)]

fn main() {
    if_else();
    if_else_if_else();
    if_in_a_let_statement();
    loops();
    if_let_example();
}

fn if_else() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_else_if_else() {
    let number = 6;
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else {
        println!("{} is not divisible by 4 or 3", number);
    }
}

fn if_in_a_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    //Non-matching return types: Err
    //let number = if condition {
    //    5
    //} else {
    //    "six"
    //};

    println!("if in a let produced {}", number);
}

fn loops() {
    let mut i: isize = 0;

    while i < 3 {
        println!("i: {}", i);
        i += 1;
    }

    let mut j: isize = 10;

    loop {
        println!("{}", j);
        j -= 1;
        if j == 0 {
            println!("LIFTOFF!!!!");
            break;
        }
    }

    let coolness = ["not epic", "very epic", "gamer"];

    for level in coolness.iter() {
        println!("{}", level);
    }

    for (index, level) in coolness.iter().enumerate() {
        println!("{}: {}", index, level);
    }

    for n in (1..4).rev() {
        println!("{}", n);
    }
    println!("LIFTOFF!!!!");
}

fn if_let_example() {
    // Consider the following potential implementation:
    //let some_u8_value = Some(0u8);
    //match some_u8_value {
    //    Some(3) => println!("three"),
    //    _ => (),
    //}
    // What if we only want to execute if a certain case, no execution if not like the above,
    // without all the boilerplate of the match expression?
    // Solution: Use if let:
    let mut some_count = 0;
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        some_count += 1;
    }
}
