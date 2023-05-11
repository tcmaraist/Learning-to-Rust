fn main() {
    let mut count = 0;
    //specifying a loop label to disambiguate loops
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping with a WHILE loop
    // potentially dangerous b/c the program can panic if it runs off the end of the array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // looping with a FOR loop
    // FOR loop is safer in this scenario because it stops at the end of the array
    let b = [10, 20, 30, 40, 50];

    for element in b {
        println!("the value is: {element}");
    }
}
