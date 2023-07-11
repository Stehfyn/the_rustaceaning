fn main() {
    if_else();
    if_else_if_else();
    if_in_a_let_statement();
    loops();
}

fn if_else() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }
}

fn if_else_if_else() {
    let number = 6;
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 ==0 {
        println!("{} is divisible by 3", number);
    } else {
        println!("{} is not divisible by 4 or 3", number);
    }
}

fn if_in_a_let_statement() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

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