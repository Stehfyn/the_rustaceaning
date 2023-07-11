//! ch1: Convert temps between Fahrenheit and Celsius
//! ch2: Generate the nth Fibonacci number
//! ch3: Print the lyrics to the Christmas Carol "Twelve Days" taking advantage of repetition

fn main() {
    ch1();
    ch2();
    ch3();
}

fn f2c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c2f(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

fn temps_is_equal_f64(x: f64, y: f64) -> bool {
    //can't use f64::EPSILON as error accumulated may exceed it
    const EPSILON_: f64 = 1e-3;
    (y < (x + EPSILON_)) && (y > (x - EPSILON_))
}

fn ch1() {
    let f: f64 = 72.0;
    println!("Challenge 1");
    println!("\u{00B0}F to \u{00B0}C: {:.2}\u{00B0}F | {:.2}\u{00B0}C",  &f, &f2c(f));
    println!("\u{00B0}C to \u{00B0}F: {:.2}\u{00B0}C | {:.2}\u{00B0}F", &f2c(f), &c2f(f2c(f)));
    println!("");
    assert!(temps_is_equal_f64(f, c2f(f2c(f))));
}

fn fib (n: usize) -> usize {
    if (n == 0) || (n == 1) {
        n
    } else {
        &fib(n - 1) + &fib(n - 2)
    }
}

fn ch2() {
    let n = 10;
    let fib_seq = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    for (i, _) in fib_seq.iter().enumerate() {
        assert!(fib(i) == fib_seq[i]);
    }

    println!("Challenge 2");
    println!("nth fibonacci number (n={}): {}", n, fib(n));
    println!("");
}

fn ch3() {
    const SUFFIX_MAP: [&str; 13] = ["" , "st", "nd", "rd", "th", "th", "th", "th", "th", "th", "th", "th", "th"];
    const GIFT_MAP: [&str; 13] = ["", "and a partridge in a pear tree!", "turtle doves", 
    "French hens", "calling birds", "golden rings!!!", "geese a laying", "swans a-swimming", 
    "maids a-milking", "ladies dancing", "lords a-leaping", "pipers piping", "drummers drumming"];

    println!("Challenge 3");
    for i in 1..13 {
        println!("On the {}{} day of Christmas my True Love gave to me...", i, SUFFIX_MAP[i]);
        for j in (1..i+1).rev() {
            let tabs: usize = i - (j - 1);
            if j != 1 && j != 5 {
                println!("{} {} {}...", &"  ".repeat(tabs), &j, &GIFT_MAP[j]);
            } else if j == 5 {
                 println!("{} {} {}", &"  ".repeat(tabs), &j, &GIFT_MAP[j]);
            } else if i == 1{
                println!("{} A {}", &"  ".repeat(tabs), &GIFT_MAP[j][6..])
            } else {
                println!("{} {}", &"  ".repeat(tabs), &GIFT_MAP[j])
            }
        }
    }
}