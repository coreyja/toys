use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    let computer_choice = choices.choose(&mut thread_rng()).unwrap();

    println!("Welcome to Rock, Paper, Scissors!");
    println!("Enter your choice: ");

    let mut player_choice = String::new();
    io::stdin().read_line(&mut player_choice).unwrap();
    let player_choice = player_choice.trim().to_lowercase();

    let winner = determine_winner(&player_choice, &computer_choice);
    println!("{}", winner);
    println!("You chose: {}", player_choice);
    println!("Computer chose: {:?}", computer_choice);
}

fn determine_winner<'a>(player: &'a str, computer: &'a Choice) -> &'a str {
    match (player, computer) {
        ("rock", Choice::Scissors) | ("scissors", Choice::Paper) | ("paper", Choice::Rock) => {
            "You win!"
        }
        ("scissors", Choice::Rock) | ("paper", Choice::Scissors) | ("rock", Choice::Paper) => {
            "Computer wins!"
        }
        ("rock", Choice::Rock) | ("paper", Choice::Paper) | ("scissors", Choice::Scissors) => {
            "It's a tie!"
        }
        _ => "Invalid input. Choose between rock, paper, and scissors.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_wins() {
        assert_eq!(determine_winner("rock", &Choice::Scissors), "You win!");
    }

    #[test]
    fn test_computer_wins() {
        assert_eq!(
            determine_winner("scissors", &Choice::Rock),
            "Computer wins!"
        );
    }

    #[test]
    fn test_tie() {
        assert_eq!(determine_winner("rock", &Choice::Rock), "It's a tie!");
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(
            determine_winner("invalid", &Choice::Rock),
            "Invalid input. Choose between rock, paper, and scissors."
        );
    }
}
