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
    println!("wnw: {:?}", wnw)
}
