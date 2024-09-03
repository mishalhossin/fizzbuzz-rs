use std::io;

fn main() {
    print!("Number for FizzBuzz: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();

    match trimmed_input.parse::<i32>() {
        Ok(num) => {
            if num % 3 == 0 && num % 5 == 0 {
                println!("FizzBuzz");
            } else if num % 3 == 0 {
                println!("Fizz");
            } else if num % 5 == 0 {
                println!("Buzz");
            } else {
                println!("{}", num);
            }
        }
        Err(_) => println!("Not a valid number"),
    }
}
