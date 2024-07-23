use rand::Rng;
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    Human,
    Computer,
}

#[derive(Clone)]
struct Game {
    board: [Option<Player>; 9],
    current_player: Player,
}

impl Game {
    const BOARD_SIZE: usize = 9;
    const BOARD_SIDE: usize = 3;
    const SCORE_WIN: i32 = 10;
    const SCORE_LOSE: i32 = -10;
    const SCORE_DRAW: i32 = 0;
    const SCORE_CONTINUE: i32 = -1;

    fn new() -> Self {
        Game {
            board: [None; Self::BOARD_SIZE],
            current_player: Player::Human,
        }
    }

    fn print_board(&self) {
        for i in 0..Self::BOARD_SIDE {
            for j in 0..Self::BOARD_SIDE {
                match self.board[i * Self::BOARD_SIDE + j] {
                    Some(Player::Human) => print!("X"),
                    Some(Player::Computer) => print!("O"),
                    None => print!("."),
                }
                if j < Self::BOARD_SIDE - 1 {
                    print!(" | ");
                }
            }
            println!();
            if i < Self::BOARD_SIDE - 1 {
                println!("---------");
            }
        }
        println!();
    }

    fn make_move(&mut self, position: usize) -> bool {
        if position < Self::BOARD_SIZE && self.board[position].is_none() {
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

    fn winning_combinations() -> Vec<[usize; 3]> {
        let mut combinations = Vec::new();

        // Horizontal rows
        for i in 0..Self::BOARD_SIDE {
            combinations.push([
                i * Self::BOARD_SIDE,
                i * Self::BOARD_SIDE + 1,
                i * Self::BOARD_SIDE + 2,
            ]);
        }

        // Vertical columns
        for i in 0..Self::BOARD_SIDE {
            combinations.push([
                i,
                i + Self::BOARD_SIDE,
                i + 2 * Self::BOARD_SIDE,
            ]);
        }

        // Diagonals
        combinations.push([0, 4, 8]);
        combinations.push([2, 4, 6]);

        combinations
    }

    fn check_winner(&self) -> Option<Player> {
        let winning_combinations = Self::winning_combinations();

        for combo in winning_combinations.iter() {
            if let (Some(player), Some(b), Some(c)) = (
                self.board[combo[0]],
                self.board[combo[1]],
                self.board[combo[2]],
            ) {
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

    fn evaluate_board(&self) -> i32 {
        if let Some(winner) = self.check_winner() {
            match winner {
                Player::Computer => Self::SCORE_WIN,
                Player::Human => Self::SCORE_LOSE,
            }
        } else if self.is_full() {
            Self::SCORE_DRAW
        } else {
            Self::SCORE_CONTINUE
        }
    }

    fn minimax(&self, is_maximizing: bool) -> i32 {
        let score = self.evaluate_board();

        if score != Self::SCORE_CONTINUE {
            return score;
        }

        let mut best_score = if is_maximizing {
            std::i32::MIN
        } else {
            std::i32::MAX
        };

        for i in 0..Self::BOARD_SIZE {
            if self.board[i].is_none() {
                let mut new_game = self.clone();
                new_game.board[i] = Some(if is_maximizing {
                    Player::Computer
                } else {
                    Player::Human
                });
                let score = new_game.minimax(!is_maximizing);
                best_score = if is_maximizing {
                    best_score.max(score)
                } else {
                    best_score.min(score)
                };
            }
        }

        best_score
    }

    fn get_best_move(&self) -> usize {
        let mut best_score = std::i32::MIN;
        let mut best_move = 0;

        for i in 0..Self::BOARD_SIZE {
            if self.board[i].is_none() {
                let mut new_game = self.clone();
                new_game.board[i] = Some(Player::Computer);
                let score = new_game.minimax(false);
                if score > best_score {
                    best_score = score;
                    best_move = i;
                }
            }
        }

        best_move
    }

    fn get_computer_move(&self) -> usize {
        self.get_best_move()
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        game.print_board();

        if let Some(winner) = game.check_winner() {
            println!(
                "{} wins!",
                if winner == Player::Human {
                    "You"
                } else {
                    "Computer"
                }
            );
            break;
        }

        if game.is_full() {
            println!("It's a draw!");
            break;
        }

        let position = if game.current_player == Player::Human {
            print!("Enter your move (0-{}): ", Game::BOARD_SIZE - 1);
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
