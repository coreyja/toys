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
        ("rock", "rock") | ("paper", "paper") | ("scissors", "scissors") => "It's a tie!",
        _ => "Invalid input. Choose between rock, paper, and scissors.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_wins() {
        assert_eq!(determine_winner("rock", "scissors"), "You win!");
    }

    #[test]
    fn test_computer_wins() {
        assert_eq!(determine_winner("scissors", "rock"), "Computer wins!");
    }

    #[test]
    fn test_tie() {
        assert_eq!(determine_winner("rock", "rock"), "It's a tie!");
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(determine_winner("invalid", "rock"), "Invalid input. Choose between rock, paper, and scissors.");
    }
}
