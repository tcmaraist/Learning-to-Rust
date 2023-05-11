fn main() {
    println!("Hello, world!");
    // it doesn't matter where the funtion is defined, only that they're defined somewhere in a scope that can be seen by the caller.
    another_function(5);

    print_labeled_measurement(5, 'h');

    let y = {
        // statemenents end with a semicolon and DO NOT return a value
        let x = 3;
        // expressions DO NOT end w/ a semicolon and DO return a value
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");

    let z = plus_one(5);

    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
