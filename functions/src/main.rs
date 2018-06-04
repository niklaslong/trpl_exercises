fn main() {
    println!("Hello, world!");

    another_function(7, 4.0);
    let five = five();

    println!("{}", five);

    let b = block();

    println!("{}", b);

    let x_plus_one = plus_one(b);
    println!("{}", x_plus_one)
}

fn another_function(x: i32, y: f64) {
    println!("x: {}, y: {}", x, y);
}

fn five() -> i32 {
    5
}

fn block() -> i32 {
    let x = {
        let y = 3;
        y + 8
    };
    x
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // if semicolon added here (x + 1;) that would be a statement (no return value) whereas without it's an expression.
}
