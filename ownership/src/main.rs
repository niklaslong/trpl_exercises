fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    println!("before func {}", s1);

    let mut s2 = s1.clone();

    s2.push_str("nana");

    take_ownership(s1);

    println!("after func {}", s2);

    // borrowing:

    let s3 = String::from("hi");

    calculate_len(&s3);

    println!("{}", s3);

    let w = word();
    println!("{}", w);

    let s4 = "this is my string";

    let f_w = first_word(s4);

    println!("{}", s4);
    println!("{}", f_w)
}

fn take_ownership(some_string: String) {
    println!("from function scope {}", some_string)
}

fn calculate_len(string: &String) -> usize {
    string.len()
}

fn word() -> &'static str {
    "hello"
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
