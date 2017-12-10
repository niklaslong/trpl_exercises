fn main() {
    let mut x = 6; //when mut keyword is not used - compiler throws "re-assignment of immutable variable" error
    println!("The variable is {}", x);

    x = 5;
    println!("The variable is now {}", x);
}
