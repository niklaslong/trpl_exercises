fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

struct ImportandExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let number_list = vec![10, 20, 30, 15, 12];

    let result = largest_i32(&number_list);
    let result2 = largest_copy(&number_list);
    println!("The largest number is {} and {}", result, result2);

    let char_list = vec!['y', 'v', 'h', '3'];

    let result = largest_char(&char_list);
    let result2 = largest_copy(&char_list);
    println!("The largest char is {} and {}", result, result2);

    let wnw = Point { x: 5, y: "z" };
    println!("wnw: {:?}", wnw);

    let string1 = String::from("qwert");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    let novel = String::from("Call me. blabla ablalbaalbss");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportandExcerpt {
        part: first_sentence,
    };

    let result2 = longest_with_an_announcement(string1.as_str(), string2, "heyo");

    println!("{} - {}", i.part, result2)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
