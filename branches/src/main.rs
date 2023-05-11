fn main() {
    let number = 6;
    //returns "number is divisible by 3" b/c Rust only executes the block for the first true condition, and it doesn't check the rest.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = false;
    let conditional_number = if condition { 5 } else { 6 };

    println!("The value of conditional_number is: {conditional_number}");
}
