use std::io::{self, Write};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    Human,
    Computer,
}

struct Game {
    board: [Option<Player>; 9],
    current_player: Player,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [None; 9],
            current_player: Player::Human,
        }
    }

    fn print_board(&self) {
        for i in 0..3 {
            for j in 0..3 {
                match self.board[i * 3 + j] {
                    Some(Player::Human) => print!("X"),
                    Some(Player::Computer) => print!("O"),
                    None => print!("."),
                }
                if j < 2 {
                    print!(" | ");
                }
            }
            println!();
            if i < 2 {
                println!("---------");
            }
        }
        println!();
    }

    fn make_move(&mut self, position: usize) -> bool {
        if position < 9 && self.board[position].is_none() {
            self.board[position] = Some(self.current_player);
            self.current_player = match self.current_player {
                Player::Human => Player::Computer,
                Player::Computer => Player::Human,
            };
            true
        } else {
            false
        }
    }

    fn check_winner(&self) -> Option<Player> {
        let winning_combinations = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], 
            [0, 3, 6], [1, 4, 7], [2, 5, 8], 
            [0, 4, 8], [2, 4, 6],           
        ];

        for combo in winning_combinations.iter() {
            if let (Some(player), Some(b), Some(c)) = (self.board[combo[0]], self.board[combo[1]], self.board[combo[2]]) {
                if player == b && player == c {
                    return Some(player);
                }
            }
        }

        None
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|&cell| cell.is_some())
    }

    fn get_computer_move(&self) -> usize {
        let mut rng = rand::thread_rng();
        loop {
            let position = rng.gen_range(0..9);
            if self.board[position].is_none() {
                return position;
            }
        }
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        game.print_board();

        if let Some(winner) = game.check_winner() {
            println!("{} wins!", if winner == Player::Human { "You" } else { "Computer" });
            break;
        }

        if game.is_full() {
            println!("It's a draw!");
            break;
        }

        let position = if game.current_player == Player::Human {
            print!("Enter your move (0-8): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            input.trim().parse::<usize>().unwrap_or(9)
        } else {
            let computer_move = game.get_computer_move();
            println!("Computer chooses position {}", computer_move);
            computer_move
        };

        if !game.make_move(position) {
            println!("Invalid move. Try again.");
        }
    }
}
