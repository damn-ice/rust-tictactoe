use std::io::{self, Write};

mod ascii_vars;
use ascii_vars as ascii;

/// Represents a player's game piece, either X or O...
enum GamePiece {
    X,
    O,
}

impl GamePiece {
    // Set lifetime static because board outlives play game...
    // Also, we could just add the board to the play game function...
    /// Returns the string representation of the game piece.
    fn as_str(&self) -> &'static str {
        match self {
            GamePiece::X => "X",
            GamePiece::O => "O",
        }
    }
    /// Switches the current player's game piece to the opponent's piece.
    fn switch(&self) -> GamePiece {
        match self {
            GamePiece::X => GamePiece::O,
            GamePiece::O => GamePiece::X,
        }       
    }
}

/// Generates and prints the game board.
fn generate_board(board: &[&str; 9]) {
    println!("{} | {} | {} ", board[0], board[1], board[2]);
    println!("{} | {} | {} ", board[3], board[4], board[5]);
    println!("{} | {} | {} ", board[6], board[7], board[8]);
}

/// Gets the user's input for their next move.
fn get_user_input(board: &[&str; 9], player: &GamePiece) -> Result<usize, &'static str> {
    println!("");
    generate_board(&board);
    println!("");
    print!("PLAYER {} turn", player.as_str());
    println!("");
    println!("From the above available numbers on the board, kindly choose a number to play: ");
    println!("");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: Result<usize, _> = input.trim().parse();
    match input {
        Ok(num) if num >= 1 && num <= 9  && board[num - 1] != "X" && board[num - 1] != "O" => Ok(num),
        _ => Err("Invalid input: Please enter a number between 1 and 9 that has not been selected. "),
    }

}


/// Clears the screen using ANSI escape codes.
fn clear_screen() {
    print!("\x1B[2J\x1B[H"); // ANSI escape codes to clear screen and move cursor to beginning...
    io::stdout().flush().expect("Failed to flush stdout");
}

/// Checks if a player has won the game.
fn check_winner(board: &[&str; 9], player: &GamePiece) -> bool {
    // Check rows
    for i in 0..3 {
        if board[i * 3] == player.as_str() && board[i * 3 + 1] == player.as_str() && board[i * 3 + 2] == player.as_str() {
            return true;
        }
    }

    // Check columns
    for i in 0..3 {
        if board[i] == player.as_str() && board[i + 3] == player.as_str() && board[i + 6] == player.as_str() {
            return true;
        }
    }

    // Check diagonals
    if board[0] == player.as_str() && board[4] == player.as_str() && board[8] == player.as_str() {
        return true;
    }
    if board[2] == player.as_str() && board[4] == player.as_str() && board[6] == player.as_str() {
        return true;
    }

    false
}

/// Prints ASCII art with styling.
fn ascii_art_print(art: &str) {
    println!("\x1B[2J\x1B[H"); // Clear screen and move cursor to the beginning
    println!("\x1B[38;5;82m\x1B[5m");
    println!("{}", art);
    println!("\x1B[0m"); // Reset style to default
}

/// Starts and manages the tic-tac-toe game loop.
pub fn play_game(board: &mut [&str; 9]) {
    clear_screen();
    let mut player = GamePiece::X;
    let mut count = 0;
    loop {
        let x = get_user_input(&board, &player);
        match x {
            Ok(num) => {
                clear_screen();
                board[num - 1] = player.as_str();
                count += 1;

                if check_winner(&board, &player) {
                    generate_board(&board);
                    match player {
                        GamePiece::O => ascii_art_print(ascii::ASCII_PLAYER_O),
                        GamePiece::X => ascii_art_print(ascii::ASCII_PLAYER_X),
                    }
                    break;
                } else if count == 9 {
                    generate_board(&board);
                    ascii_art_print(ascii::ASCII_PLAYER_TIE);
                    break;
                }

                player = player.switch();
                continue;
            }
            Err(e) => {
                clear_screen();
                print!("{e}");
                continue;
            }
        }
    }
}

