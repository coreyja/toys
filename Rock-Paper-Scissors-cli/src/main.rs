use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    let choices = ["rock", "paper", "scissors"];
    let computer_choice = choices.choose(&mut thread_rng()).unwrap();

    println!("Welcome to Rock, Paper, Scissors!");
    println!("Enter your choice: ");

    let mut player_choice = String::new();
    io::stdin().read_line(&mut player_choice).unwrap();
    let player_choice = player_choice.trim().to_lowercase();

    
    let winner = determine_winner(&player_choice, &computer_choice);
    println!("{}", winner);

    println!("You chose: {}", player_choice);
    println!("Computer chose: {}", computer_choice);
}

fn determine_winner<'a>(player: &'a str, computer: &'a str) -> &'a str {
    match (player, computer) {
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => "You win!",
        ("scissors", "rock") | ("paper", "scissors") | ("rock", "paper") => "Computer wins!",
        _ => "It's a tie!",
    }
}
