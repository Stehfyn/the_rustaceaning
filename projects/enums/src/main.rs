#![allow(unused)]

fn main() {
    basic_enum_example();
    basic_struct_enum_example();
    basic_concise_struct_enum_example();
    generic_option_usage();
    match_expression_example();
    match_pattern_bind_example();
    more_generic_option_usage();
    generic_match_exhaustion_example();
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {}

fn basic_enum_example() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn basic_struct_enum_example() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddr_ {
    V4(String),
    V6(String),
}

// or ...

enum IpAddr__ {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr__ {
    fn print(&self) -> String {
        match self {
            IpAddr__::V4(a, b, c, d) => return format!("{}.{}.{}.{}", a, b, c, d),
            IpAddr__::V6(s) => return format!("{}", s),
        }
    }
}

fn basic_concise_struct_enum_example() {
    let home = IpAddr_::V4(String::from("127.0.0.1"));
    let loopback = IpAddr_::V6(String::from("::1"));

    let home = IpAddr__::V4(127, 0, 0, 1);
    let loopback = IpAddr__::V6(String::from("::1"));

    println!("home: {}", home.print());
    println!("loopback: {}", loopback.print());
}

fn generic_option_usage() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn match_expression_example() {
    println!("{}", value_in_cents(Coin::Penny));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
}

enum Coin_ {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_usstate(coin: Coin_) -> u32 {
    match coin {
        Coin_::Penny => 1,
        Coin_::Nickel => 5,
        Coin_::Dime => 10,
        Coin_::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn match_pattern_bind_example() {
    value_in_cents_usstate(Coin_::Quarter(UsState::Alabama));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn more_generic_option_usage() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn generic_match_exhaustion_example() {
    let x = 10;
    match x {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("i'm exhausted!"), // could be replaced with the unit value ()
    }
}
