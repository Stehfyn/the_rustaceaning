#[allow(unused)]
pub fn create_vector() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
}

pub fn update_vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.pop();
    v.pop();
}

#[allow(unused)]
pub fn dropped_vector() {
    {
        let v = vec![1, 2, 3];
    } //v and its elements are dropped here
}

#[allow(unused)]
pub fn reading_elements() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);
    //Out of bounds: Panic
    //let dne = &v[100];
    // Safe access with get
    let dne = v.get(100);
}

pub fn mutating_vector() {
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}

// Use if exhaustive list of types exists at compile-time
enum Element {
    Int(i32),
    Float(f64),
    Text(String),
}

#[allow(unused)]
pub fn heterogenous_vector() {
    let v = vec![
        Element::Int(3),
        Element::Text(String::from("blue")),
        Element::Float(10.12),
    ];
}

#[allow(unused)]
pub fn create_string() {
    let s = String::new();
}

#[allow(unused)]
pub fn load_into_string() {
    let s = String::new();
    //to_string => any type that implements Display trait
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    //string store UTF-8 encoded data, therefore
    let s = String::from("\u{0D0B}");
    // or ...
    let s = String::from("ഋ");
}

#[allow(unused)]
pub fn append_to_string() {
    let mut s = String::new();
    s.push('\u{0B0D}');
    s.push_str("string");

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: s1 has been moved here and acan no longer be used.
}

use std::collections::HashMap;
#[allow(unused)]
pub fn create_hash_map() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

pub fn hash_map_ownership() {
    let field_name = String::from("fav color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    //Strings and owned types are moved, Copy types are copied
}

#[allow(unused)]
pub fn hash_map_access() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

pub fn hash_map_overwrite() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
}

pub fn hash_map_conditional_insert() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

pub fn hash_map_entry_dependent_update() {
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        create_vector();
        update_vector();
        dropped_vector();
        reading_elements();
        mutating_vector();
        heterogenous_vector();
        create_string();
        load_into_string();
        append_to_string();
        create_hash_map();
        hash_map_ownership();
        hash_map_access();
        hash_map_overwrite();
        hash_map_conditional_insert();
        hash_map_entry_dependent_update();
    }
}
