// This is a simple tic-tac-toe game implemented in Rust.
// It uses the minimax algorithm to determine the computer's move.
// The board is printed to the console.
// The game is played in a loop until the board is full or a player wins.


//We have a bug in this code:
//However little buddy over here 🦄 (stil need to give them a name) here to guide you where the bug is

//🦄 there's bugs in this code
//🦄 I will give hints where it might be and you have to write the correct solution




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


    // 🦄 hmmmm seems like one of the next two functions has a bug


    fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell.is_some()))
    }

    fn evaluate_board(&self) -> Option<i32> {
        if let winner = check_winner() {
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



    fn minimax(&self, is_maximizing: bool) -> (i32, Option<(usize, usize)>) {
        if let Some(score) = self.evaluate_board() {
            return (score, None);
        }

        let mut best_score = if is_maximizing { i32::MIN } else { i32::MAX };
        let mut best_move = None;

        for i in 0..Self::BOARD_SIDE {
            for j in 0..Self::BOARD_SIDE {
                if self.board[i][j].is_none() {
                    let mut new_game = self.clone();
                    new_game.board[i][j] = Some(if is_maximizing {
                        Player::Computer
                    } else {
                        Player::Human
                    });
                    let score = new_game.minimax(!is_maximizing).0;
                    if is_maximizing {
                        if score > best_score {
                            best_score = score;
                            best_move = Some((i, j));
                        }
                    } else if score < best_score {
                        best_score = score;
                        best_move = Some((i, j));
                    };
                }
            }
        }

        (best_score, best_move)
    }

    // 🦄 something seems off here

    fn get_computer_move(&self) -> (usize, usize) {
        self.minimax(false).1.unwrap()
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
        //🦄 ouuuuu seems someone forgot something around here

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

// 🦄 Hint 1: Check the `make_move` function. Is there a potential off-by-one error?
// 🦄 Hint 2: Look at the `evaluate_board` function. Is it handling all cases correctly?
// 🦄 Hint 3: In the `main` function, what happens if the user inputs an invalid move?
