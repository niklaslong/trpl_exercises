#[derive(Debug)]
enum IpAddr {
    V4(u32, u32, u32, u32),
    V6(String),
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("{:?}", state);
                25
            }
        }
    }
}

fn main() {
    let v4 = IpAddr::V4(0, 0, 0, 0);
    let v6 = IpAddr::V6(String::from("::1"));

    println!("{:?}", v4);
    println!("{:?}", v6);

    let x: Option<i32> = Some(3);
    let _absent: Option<i32> = None;

    println!("{:?}", x);

    let coin = Coin::Quarter(State::Alaska);

    let value = coin.value_in_cents();
    println!("{}", value)
}
