#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.height > other_rect.height && self.height > other_rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle { width: 20, ..rect1 };
    let rect3 = Rectangle {
        width: 10,
        height: 10,
    };

    let square = Rectangle::square(10);

    println!("{:?}", rect1);
    // useful for larger structs
    println!("{:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("The are of the square is {} square pixels", square.area());

    println!("Can rect1 contain rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 contain rect3? {}", rect2.can_hold(&rect3));

    // #[derive(Debug)] allows printing of tuples
    let tup: (i32, i32) = (2, 3);
    println!("{:?}", tup);

    // and arrays
    let a = [1, 2, 3];
    println!("{:?}", a);
}
