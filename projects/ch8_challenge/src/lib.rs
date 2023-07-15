//! ch1: Given a list of integers, use a vector and return the mean, the median, and mode of the list
//! ch2: Convert strings to pig latin
//! ch3: Using a hash map and vectors, create a text interface to allow a user to add employee names to
/// a department in a company.

pub fn floats_are_close(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}

#[derive(Debug)]
pub enum Statistic {
    Mean(f64),
    Median(f64),
    Mode(f64),
}

use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Statistics {
    mean: Statistic,
    median: Statistic,
    mode: Statistic,
}

impl PartialEq for Statistics {
    fn eq(&self, other: &Self) -> bool {
        (match (&self.mean, &other.mean) {
            (Statistic::Mean(a), Statistic::Mean(b)) => floats_are_close(*a, *b, 0.1),
            _ => false,
        }) && (match (&self.median, &other.median) {
            (Statistic::Median(a), Statistic::Median(b)) => floats_are_close(*a, *b, 0.1),
            _ => false,
        }) && (match (&self.mode, &other.mode) {
            (Statistic::Mode(a), Statistic::Mode(b)) => floats_are_close(*a, *b, 0.1),
            _ => false,
        })
    }
}

pub fn calc_mean(v: &Vec<i32>) -> Statistic {
    let mut sum = 0.0;
    for x in v.iter() {
        sum += *x as f64;
    }
    Statistic::Mean(sum / (v.len() as f64))
}

pub fn calc_median(v: &Vec<i32>) -> Statistic {
    let mut sorted = v.clone();
    sorted.sort();
    let size = sorted.len();
    if size % 2 == 0 {
        Statistic::Median((sorted[(size / 2) - 1] + sorted[size / 2]) as f64 / 2.0)
    } else {
        Statistic::Median(sorted[(size / 2) as usize] as f64)
    }
}

use std::collections::HashMap;
pub fn calc_mode(v: &Vec<i32>) -> Statistic {
    let mut mode = 0i32;
    let mut max_count = 0i32;
    let mut map = HashMap::new();
    for x in v {
        let count = map.entry(x).or_insert(0);
        *count += 1;
        if *count > max_count {
            mode = *x;
            max_count = *count;
        }
    }
    Statistic::Mode(mode as f64)
}

/// ch1: Given a list of integers, use a vector and return the mean, the median, and mode of the list
/// Note: does not handle lists with No Mode or multiple Modes
pub fn ch1(v: &Vec<i32>) -> Statistics {
    Statistics {
        mean: calc_mean(v),
        median: calc_median(v),
        mode: calc_mode(v),
    }
}

pub fn split_into_words(s: &String) -> Vec<String> {
    let mut words = Vec::new();
    for word in s.split_whitespace() {
        words.push(word.to_string());
    }
    words
}

pub fn starts_with_vowel(s: &str) -> bool {
    const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];
    for ch in VOWELS {
        if &s[..1] == ch {
            return true;
        }
    }
    return false;
}

pub fn word_to_piglatin(s: &str) -> String {
    assert!(s.len() > 0);
    let mut pl_word = String::new();
    if starts_with_vowel(&s[..1]) {
        pl_word.push_str(&s[..]);
        pl_word.push_str("-hay");
    } else {
        pl_word.push_str(&s[1..]);
        pl_word.push_str("-");
        pl_word.push_str(&s[..1]);
        pl_word.push_str("ay");
    }
    pl_word
}

/// ch2: Convert strings to pig latin => The first consonant of each word is moved to the end of the
/// word and "ay" is added, so "first" becomes "irst-fay". Words that start with a vowel have "hay"
/// added to the end instead, so "apple" becomes "apple-hay".
pub fn ch2(s: &String) -> String {
    let words = split_into_words(s);
    let mut pig_latin = String::new();
    for (i, word) in words.iter().enumerate() {
        pig_latin.push_str(&word_to_piglatin(&word[..])[..]);
        if i < words.len() - 1 {
            pig_latin.push_str(" ");
        }
    }
    pig_latin
}

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Get,
}

lazy_static! {
    static ref OPMAP: HashMap<&'static str, Operation> = {
        let mut map = HashMap::new();
        map.insert("Add", Operation::Add);
        map.insert("Get", Operation::Get);
        map
    };
}

impl Operation {
    fn from(s: &str) -> Option<Operation> {
        OPMAP.get(s).cloned()
    }
}

fn add_to_company(
    op: &Option<Operation>,
    words: &Vec<String>,
    company: &mut HashMap<String, Vec<String>>,
) {
    match op {
        Some(Operation::Add) => {
            println!("Operation recognized!");
            let department = words[3].clone();
            let employee_name = words[1].clone();
            let employees = company.entry(department).or_insert(Vec::new());
            if !employees.contains(&employee_name) {
                employees.push(employee_name);
            }
        }
        _ => println!("Can't use this operation to Add to Company!"),
    }
}

fn get_department(
    op: &Option<Operation>,
    words: &Vec<String>,
    company: &mut HashMap<String, Vec<String>>,
) {
    match op {
        Some(Operation::Get) => {
            println!("Operation recognized!");
            let department = words[1].clone();
            let employees = company.entry(department.clone()).or_insert(Vec::new());
            println!("{} Department:", department);
            for (i, e) in employees.iter().enumerate() {
                println!("{}: {}", (i + 1), e);
            }
        }
        _ => println!("Can't use this operation to Get department!"),
    }
}

fn mux_op(op: &Option<Operation>, words: &Vec<String>, company: &mut HashMap<String, Vec<String>>) {
    match op {
        Some(Operation::Add) => {
            assert!(words.len() == 4);
            add_to_company(&op, &words, company);
        }
        Some(Operation::Get) => {
            assert!(words.len() == 2);
            get_department(&op, &words, company);
        }
        _ => println!("Unable to mux op!"),
    }
}

use std::io::{self, Write}; // add `Write` trait for `stdout().flush()`
pub fn ch3_interactive() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let words = split_into_words(&String::from(&(*input)[..]));

        let word_count = words.len();
        if (word_count == 2) || (word_count == 4) {
            let op = Operation::from(&(words[0])[..]);
            mux_op(&op, &words, &mut company);
        } else {
            if words[0].to_lowercase() == "exit" {
                println!("Exiting!");
                break;
            } else {
                println!("wc: {}", word_count);
                println!("Error: Invalid Operation");
            }
        }
    }
}

/// ch3: Using a hash map and vectors, create a text interface to allow a user to add employee names to
/// a department in a company. For example, "Add Sally to Engineering" or "Add Amir to Sales". Then
/// let the user retrieve a list of all people in a department or all people in the company by
/// department, sorted alphabetically.
pub fn ch3(input: &Vec<&str>) -> HashMap<String, Vec<String>> {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let words = split_into_words(&String::from(&(*line)[..]));
        assert!(words.len() >= 2);
        let op = Operation::from(&(words[0])[..]);
        mux_op(&op, &words, &mut company);
    }
    return company;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch1() {
        let input = vec![13, 18, 13, 14, 13, 16, 14, 21, 13];
        let answer = Statistics {
            mean: Statistic::Mean(15.0),
            median: Statistic::Median(14.0),
            mode: Statistic::Mode(13.0),
        };
        assert!(answer == ch1(&input));
    }

    #[test]
    fn test_ch2() {
        let input = String::from("first apple");
        let answer = String::from("irst-fay apple-hay");
        assert!(answer == ch2(&input));
    }

    #[test]
    fn test_ch3() {
        let input = vec!["Add Sally to Engineering", "Add Amir to Sales", "Get Sales"];
        let mut answer = HashMap::new();
        answer.insert("Engineering".to_string(), vec!["Sally".to_string()]);
        answer.insert("Sales".to_string(), vec!["Amir".to_string()]);
        assert!(answer == ch3(&input));
    }
}
