#![allow(unused)]

fn main() {
    basic_struct_example();
    basic_struct_update_example();
    basic_tuple_struct_example();
    basic_unitlike_struct_example();
    example_program();
    struct_with_derived_traits_example();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// There exists shorthand for struct field initialization (email: email, -> email,) etc..
fn build_user(email: &String, username: &String) -> User {
    User {
        active: true,
        sign_in_count: 1,
        email: email.clone(),
        username: username.clone(),
    }
}

fn basic_struct_example() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
        email: String::from("someone@example.com"),
        username: String::from("someone@example.com"),
    };

    user1.email = String::from("another@example.com");

    let create_email = String::from("build@example.com");
    let user2 = build_user(&create_email, &create_email);
}

fn basic_struct_update_example() {
    let create_email = String::from("build@example.com");
    let user1 = build_user(&create_email, &create_email);
    let user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone@example.com"),
        ..user1
    };
}

fn basic_tuple_struct_example() {
    // Kinda like a cute lil typedef!
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("r: {}, g: {}, b: {}", black.0, black.1, black.2);
}

fn basic_unitlike_struct_example() {
    //come back at chapter 10!
}

struct Rect(u32, u32);

fn area1(r: &Rect) -> u32 {
    r.0 * r.1
}

// or ...

fn area2(r: (u32, u32)) -> u32 {
    r.0 * r.1
}

// or ...

struct RectWithNames {
    width: u32,
    height: u32,
}

fn area3(r: &RectWithNames) -> u32 {
    r.width * r.height
}

// Calculates the area of a rectangle
fn example_program() {
    let width1 = 30;
    let width2 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(&Rect(30, 50))
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area2((30, 50))
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&RectWithNames {
            width: 30,
            height: 50
        })
    );
}

#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.w > other.w) && (self.h > other.h)
    }

    // Associated functions
    fn square(x: u32) -> Rectangle {
        Rectangle { w: x, h: x }
    }
}

fn struct_with_derived_traits_example() {
    let r1 = Rectangle { w: 30, h: 50 };
    println!("{:?}", r1); // Explicitly use Debug trait with :?
    println!("{:#?}", r1); // Indented!
    println!("The area of the rectangle is {} square pixels.", r1.area()); // Indented!
    let r2 = Rectangle { w: 10, h: 10 };
    println!("r1 can hold r2: {}", r1.can_hold(&r2)); // Indented!
    let x = 7;
    println!("square of side_length {},:{:#?}", x, Rectangle::square(x));
}
