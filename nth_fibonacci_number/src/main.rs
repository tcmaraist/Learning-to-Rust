use std::io;

fn main() {
    println!("Print the Nth number of the Fibonacci Sequence.");

    loop {
        println!("Please input the number you wish to calculate");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let f = fibonacci(input - 1);
        println!("the fibonacci of {input} is {f}.");
        break;
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
