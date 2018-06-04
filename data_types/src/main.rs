fn main() {
    // scalar types: (represent a single value)

    let x = 2.0;
    println!("{}", x);

    // default floating-point type is f64
    let y: f32 = 4.0;
    println!("{}", y);

    let sum = x + y;
    println!("{}", sum);

    // the type here is the same as the y variable. Setting a f64 as type would raise mismatched types error (expected f64, found f32)
    let diff: f32 = x - y;
    println!("{}", diff);

    let quotient: f32 = x / y;
    println!("{}", quotient);

    let remainder: f32 = x % y;
    println!("{}", remainder);

    let t: bool = true;
    println!("{}", t);

    let f = false;
    println!("{}", f);

    // single quotes are chars (double quotes are strings)
    let c: char = 'c';
    println!("{}", c);

    let pi = 'π';
    println!("{}", pi);

    // compound types: (groups multiple values into 1 type)

    let tup: (i32, char, bool) = (-3, 'π', true);
    // destructuring using pattern matching:
    let (three, pi, t) = tup;
    println!("three: {}, pi: {}, t: {}", three, pi, t);

    let x: (f32, i32, char) = (3.0, -4, 'µ');
    // using indexes
    let float: f32 = x.0;
    let int: i32 = x.1;
    let mu: char = x.2;

    println!("float: {}, int: {}, mu: {}", float, int, mu);

    let a = [1, 2, 3, 4];

    let first = a[0];
    let second = a[1];

    println!("first: {}, second: {}", first, second);

    let chars = ['a', 'b', 'c'];
    let [a, b, c] = chars;

    println!("a: {}, b: {}, c: {}", a, b, c);
}
