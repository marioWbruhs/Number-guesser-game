use rand::Rng;
use std::cmp::Ordering;
use std::io;
use math::round;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); //generating the random number
    let mut guesses = 0;                                      //creating a variable to hold the total amount of tries
    let mut guess = 50 as f64;                                //creating a variable for the guess the bot is gonna submit

    //the loop:)
    loop {
        println!("Please input your guess.");

        guesses += 1; //adds one to the amount of tries everytime the bot loops the program
        
        println!("You guessed: {}", guess); //prints the guess

        match (guess as i8).cmp(&secret_number) { 
            /*guessing algirithm:
                I used binary search to find the correct number
                If the guess is smaller than the secret_number variable, i add (secret_number_max_bound - guess) / 2 to the guess
                If its smaller, i check if its an even number. If it is, i just devide it by 2. If its not, i devide it by 2 and add 1 to it
            */
            Ordering::Less => { //check if the guess is smaller than the random number generated at the start of the program
                println!("Too small!");
                guess = (guess as i8 + (100 - guess as i8) as i8 / 2) as f64;
                println!("The guess is now : {}", guess);
            },
            Ordering::Greater => { //check if the guess is bogger than the random number generated at the start of the program
                println!("Too big!");
                
                if(guess % 2 as f64 == 0 as f64){
                    guess = guess as f64 / 2 as f64;
                }
                else{
                    guess = (guess as f64 / 2 as f64) + 1 as f64;
                }
                //guess = round::ceil((guess as u8/2) as f64, 10);
                println!("The guess is now : {}", guess);
            },
            Ordering::Equal => { //check if the guess is equal to the random number generated at the start of the program
                println!("You win!");
                println!("You guessed the number {} in {} tries!", secret_number, guesses); //prints the number generated at the beggining and the amount of tries 
                break;
            }
        }
    }
}