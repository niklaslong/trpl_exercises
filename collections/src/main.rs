fn main() {
    vectors();
    strings();
    hash_maps();
    let mean = vector_mean(vec![1, 2, 3]);
    println!("{}", mean);
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3, 4];
    let third: &i32 = &v2[2];
    let fourth: Option<&i32> = v2.get(3);

    println!("v is: {:?} and v2 is: {:?}", v, v2);
    println!("third is: {} and fourth is: {:?}", third, fourth);

    for i in &mut v {
        *i += 50;
        println!("{}", i)
    }

    println!("{:?}", v);

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(2.3),
        SpreadSheetCell::Text(String::from("hello world")),
    ];

    println!("{:?}", row);
}

fn strings() {
    let mut s = String::from("foo");
    s.push_str(" bar");

    let mut s2 = "lo".to_string();
    s2.push('l');

    let s3 = s2 + " hi" + " woah " + &s;
    println!("{}", s3);

    // format! is easier for concatinating multiple strings

    format!("{}-{}", s, s3);
}

use std::collections::HashMap;

fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let new_hm: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", new_hm);

    let field_name = String::from("favourite color");
    let field_value = String::from("green");

    let mut map = HashMap::new();
    // The values that the references point to must be valid for at least as long as the hash map is valid.
    map.insert(&field_name, field_value);

    let color = map.get(&field_name);
    println!("{:?}", color);

    let text = "this is really cool, I think this is cool";

    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", text_map);
}

fn vector_mean(vector: Vec<i32>) -> i32 {
    let mut mean = 0;

    for i in vector {
        mean += i;
    }

    mean / vector.len()
}
