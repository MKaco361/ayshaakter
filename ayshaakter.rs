use std::io;
use rand::Rng;

fn guess_game() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut attempts = 0;
    
    println!("Guess the number (1-10)");
    
    while attempts < 3 {
        println!("Enter your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        attempts += 1;
        
        if guess < secret_number {
            println!("Too low!");
        } else if guess > secret_number {
            println!("Too high!");
        } else {
            println!("You win! Found in {} attempts", attempts);
            return;
        }
    }
    
    println!("Game over! The number was {}", secret_number);
}

fn dice_roll() {
    let dice1 = rand::thread_rng().gen_range(1..=6);
    let dice2 = rand::thread_rng().gen_range(1..=6);
    let total = dice1 + dice2;
    
    println!("Dice 1: {}", dice1);
    println!("Dice 2: {}", dice2);
    println!("Total: {}", total);
    
    if dice1 == dice2 {
        println!("Double!");
    }
    
    if total == 7 {
        println!("Lucky 7!");
    }
}

fn calculator() {
    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a number!");
    
    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please enter a number!");
    
    println!("Sum: {}", num1 + num2);
}

fn main() {
    loop {
        println!("\n1. Guess Game");
        println!("2. Dice Roll");
        println!("3. Calculator");
        println!("4. Exit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        match choice.trim() {
            "1" => guess_game(),
            "2" => dice_roll(),
            "3" => calculator(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}