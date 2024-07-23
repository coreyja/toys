use rand::Rng;
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    Human,
    Computer,
}

#[derive(Clone)]
struct Game {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
}

impl Game {
    const BOARD_SIDE: usize = 3;
    const SCORE_WIN: i32 = 10;
    const SCORE_LOSE: i32 = -10;
    const SCORE_DRAW: i32 = 0;

    fn new() -> Self {
        Game {
            board: [[None; Self::BOARD_SIDE]; Self::BOARD_SIDE],
            current_player: Player::Human,
        }
    }

    fn print_board(&self) {
        for i in 0..Self::BOARD_SIDE {
            for j in 0..Self::BOARD_SIDE {
                match self.board[i][j] {
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

    fn make_move(&mut self, x: usize, y: usize) -> bool {
        if x < Self::BOARD_SIDE && y < Self::BOARD_SIDE && self.board[x][y].is_none() {
            self.board[x][y] = Some(self.current_player);
            self.current_player = match self.current_player {
                Player::Human => Player::Computer,
                Player::Computer => Player::Human,
            };
            true
        } else {
            false
        }
    }

    fn winning_combinations() -> Vec<[(usize, usize); 3]> {
        let mut combinations = Vec::new();

        // Horizontal rows
        for i in 0..Self::BOARD_SIDE {
            combinations.push([(i, 0), (i, 1), (i, 2)]);
        }

        // Vertical columns
        for i in 0..Self::BOARD_SIDE {
            combinations.push([(0, i), (1, i), (2, i)]);
        }

        // Diagonals
        combinations.push([(0, 0), (1, 1), (2, 2)]);
        combinations.push([(0, 2), (1, 1), (2, 0)]);

        combinations
    }

    fn check_winner(&self) -> Option<Player> {
        let winning_combinations = Self::winning_combinations();

        for combo in winning_combinations.iter() {
            if let (Some(player), Some(b), Some(c)) = (
                self.board[combo[0].0][combo[0].1],
                self.board[combo[1].0][combo[1].1],
                self.board[combo[2].0][combo[2].1],
            ) {
                if player == b && player == c {
                    return Some(player);
                }
            }
        }

        None
    }

    fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell.is_some()))
    }

    fn evaluate_board(&self) -> Option<i32> {
        if let Some(winner) = self.check_winner() {
            match winner {
                Player::Computer => Some(Self::SCORE_WIN),
                Player::Human => Some(Self::SCORE_LOSE),
            }
        } else if self.is_full() {
            Some(Self::SCORE_DRAW)
        } else {
            None
        }
    }

    fn minimax(&self, is_maximizing: bool) -> i32 {
        if let Some(score) = self.evaluate_board() {
            return score;
        }

        let mut best_score = if is_maximizing {
            std::i32::MIN
        } else {
            std::i32::MAX
        };

        for i in 0..Self::BOARD_SIDE {
            for j in 0..Self::BOARD_SIDE {
                if self.board[i][j].is_none() {
                    let mut new_game = self.clone();
                    new_game.board[i][j] = Some(if is_maximizing {
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
        }

        best_score
    }

    fn get_best_move(&self) -> (usize, usize) {
        let mut best_score = std::i32::MIN;
        let mut best_move = (0, 0);

        for i in 0..Self::BOARD_SIDE {
            for j in 0..Self::BOARD_SIDE {
                if self.board[i][j].is_none() {
                    let mut new_game = self.clone();
                    new_game.board[i][j] = Some(Player::Computer);
                    let score = new_game.minimax(false);
                    if score > best_score {
                        best_score = score;
                        best_move = (i, j);
                    }
                }
            }
        }

        best_move
    }

    fn get_computer_move(&self) -> (usize, usize) {
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

        let (x, y) = if game.current_player == Player::Human {
            print!("Enter your move (x, y): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim().trim_matches(|c| c == '(' || c == ')');
            let mut parts = input.split(',');
            let x = parts
                .next()
                .unwrap_or_default()
                .trim()
                .parse::<usize>()
                .unwrap_or(0);
            let y = parts
                .next()
                .unwrap_or_default()
                .trim()
                .parse::<usize>()
                .unwrap_or(0);

            (x - 1, y - 1)
        } else {
            let (computer_move_x, computer_move_y) = game.get_computer_move();
            println!(
                "Computer chooses position ({}, {})",
                computer_move_x + 1,
                computer_move_y + 1
            );
            (computer_move_x, computer_move_y)
        };

        if !game.make_move(x, y) {
            println!("Invalid move. Try again.");
        }
    }
}
