use std::io; //get input-output functions from the standard library
use std::cmp::Ordering; //enum 
use rand::Rng; //using rand as external dependency

fn main() {
    
    println!("Guess the number game!");
    
    //random number generator local to thread of execution
    //generated number between two numbers in the range
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Input your guess");
    
        //creates a mutable variable of type string
        //variables in rust are immutable by default
        let mut guess = String::new(); 
    
        //read_line method from the std::io library
        //argument guess is mutable so that it can be changed
        //& indicates it is passed as reference
        //read_line is put into the string passed
        //returns Ok or Err
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        //trim removes whitespace, parse converts string to int
        //if unable to convert to int, print content of expect
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { 
            //compare guess to secret number
            //like switch case(?)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
