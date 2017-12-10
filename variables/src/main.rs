fn main() {
    // Variable mutability and immutability
    // When mut keyword is not used - compiler throws "re-assignment of immutable variable" error

    let mut x = 6;
    println!("The variable x is: {}", x);

    x = 5;
    println!("The variable x is now: {}", x);

    // Example of "shadowing"

    let y = 8;
    println!("The variable y is: {}", y);
    let y = y + 2;
    let y = y * 3;
    println!("Tha variable y is now: {}", y);

    // Another use-case of shadowing

    let spaces = "   ";
    let _spaces = spaces.len(); // unused var (use _ to avoid compiler warnings. Var can however still be called using _spaces)
    println!("Spaces: {}", _spaces)

    /*
    This construct is allowed because the first spaces variable is a string type,
    and the second spaces variable, which is a brand-new variable that happens to have the same name as the first one,
    is a number type. Shadowing thus spares us from having to come up with different names,
    like spaces_str and spaces_num; instead, we can reuse the simpler spaces name.
    However, if we try to use mut for this, as shown here:

    let mut spaces = "   ";
    spaces = spaces.len();

    we’ll get a compile-time error because we’re not allowed to mutate a variable’s type.
    */

}
