use std::io;
use std::io::stdout;
use rand::{thread_rng, Rng};
use crossterm::{execute, cursor};
use crossterm::terminal::{Clear, ClearType};


fn main() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();

    println!("I'm thinking of a number and you have to guess it!");
    println!("you aren't alone though...");
    println!("I also have a robot friend competing with you!\n");
    println!("Guess the number between 1 and 100!");
    println!("I'll tell you if your guess is too high or too low");

    let secret_number:u8 = thread_rng().gen_range(1..=100);
    let mut low:u8=0;
    let mut high:u8=101;

    let mut robot_guess:u8;

    loop {
        println!("Enter your guess:");
        let mut user_guess = String::new();
        let user_guess:u8 = match io::stdin().read_line(&mut user_guess) {
            Ok(_) => match user_guess.trim().parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter an integar between 1 and 100!");
                        continue;
                    },
                }
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        };

        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();

        robot_guess = thread_rng().gen_range(low + 1..high);
        println!("Your guess: {}", user_guess);
        println!("Robot guess: {}", robot_guess);
        
        if user_guess == secret_number {
            println!("You win!");
            println!("The secret number was {}", secret_number);
            break;
        }

        if robot_guess == secret_number {
            println!("The robot won!");
            println!("The secret number was {}", secret_number);
            break;
        }

        if user_guess > secret_number {
            println!("Your guess is too high!");
            
            if user_guess < high { high = user_guess };
        }
        
        if user_guess < secret_number{
            println!("Your guess is too low!");
            
            if user_guess > high { low = user_guess };
        }

        if robot_guess > secret_number {
            println!("The robots guess is too high!");

            if robot_guess < high { high = robot_guess };
        }
        
        if robot_guess < secret_number{
            println!("The robots guess is too low!");

            if robot_guess > low { low = robot_guess };
        }
    }

    println!("Thanks for playing!");
    println!("Press enter to exit...");
    let mut enter = String::from("");
    io::stdin().read_line(&mut enter).unwrap();
}
