use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    let word_list = vec!["rust", "hangman", "programming", "computer"];
    let secret_word = choose_word(&word_list);
    let mut guessed_letters = HashSet::new();
    let mut attempts_left = 6;

    println!("Welcome to Word Guess!");
    println!("The word has {} letters.", secret_word.len());

    while attempts_left > 0 {
        display_state(&secret_word, &guessed_letters, attempts_left);
        let guess = get_guess();

        if guessed_letters.contains(&guess) {
            println!("You already guessed that letter 🫠");
            continue;
        }

        guessed_letters.insert(guess);

        if secret_word.contains(guess) {
            println!("Good guess!");
        } else {
            println!("Wrong guess.");
            attempts_left -= 1;
        }

        if check_win(&secret_word, &guessed_letters) {
            println!("Wohhooooo You guessed the word: {} 🎉 ", secret_word);
            return;
        }
    }

    println!("You lost! The word was: {} ❌", secret_word);
}

fn choose_word(word_list: &[&str]) -> String {
    word_list.choose(&mut rand::thread_rng()).unwrap().to_string()
}

fn display_state(secret_word: &str, guessed_letters: &HashSet<char>, attempts_left: usize) {
    let display_word: String = secret_word
        .chars()
        .map(|letter| if guessed_letters.contains(&letter) { letter } else { '_' })
        .collect();
    
    println!("Word: {}", display_word);
    println!("Attempts left: {}", attempts_left);
}

fn get_guess() -> char {
    print!("Guess a letter: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess.trim().chars().next().unwrap()
}

fn check_win(secret_word: &str, guessed_letters: &HashSet<char>) -> bool {
    secret_word.chars().all(|c| guessed_letters.contains(&c))
}
