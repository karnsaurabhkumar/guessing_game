use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main(){
    // print the title of the number
    println!("Guess the number");

    // Creating a variable called secret number with a random number generator
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // print the line using rust macro
        println!("Please input your guess");

        let mut guess= String::new();

        //rust standard input output library to read a line
        io::stdin().
            read_line(&mut guess).
            expect("Failed to read the line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is: {}", secret_number);
                break;
            }
        }
    }
}