use std::io;

fn main() {
    println!("Convert fahrenheit to celsius!");

    loop {
        println!("Please input the temp you wish to convert.");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You input: {temp}");

        let temp_fahrenheit = temp;
        let temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
        println!("{} fahrenheit is {} celsius", temp_fahrenheit, temp_celsius);
        break;
    }

    fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }
}
