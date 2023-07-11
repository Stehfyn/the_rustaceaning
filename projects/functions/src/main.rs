#![allow(unused)]

fn main() {
    another_function(5);
    function_containing_statement();
    function_containing_statement_and_expressions();
    functions_with_return_values();
}

fn another_function(x: i32) {
    //function calls and macros are expressions
    println!("The value of x is: {}", x);
}

fn function_containing_statement() {
    let y = 6;
}

fn function_containing_statement_and_expressions() {
    //Statement assigning statement: Err
    //let x = (let y = 6);
    //let x = y = 6;

    //Statement assigning expression from {} block

    let y = {
        let x = 3;
        x + 1; // what happens here?
    };

    println!("The value of y is: {:#?}", y);

    let y = {
        let x = 3;
        x + 1 //emit ; --> like "return"
    };

     println!("The value of y is: {:#?}", y);
}

fn five() -> isize {
    5
}

fn plus_one(x: isize) -> isize {
    x + 1isize
}

fn functions_with_return_values() {
    //Note: five() and plus_one have matching types!
    println!("5 + 1 is {}", plus_one(five()));
}