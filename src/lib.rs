use std::io::{self, Write};

enum GamePiece {
    X,
    O,
}

impl GamePiece {
    // Set lifetime static because board outlives play game...
    fn as_str(&self) -> &'static str {
        match self {
            GamePiece::X => "X",
            GamePiece::O => "O",
        }
    }
    fn switch(&self) -> GamePiece {
        match self {
            GamePiece::X => GamePiece::O,
            GamePiece::O => GamePiece::X,
        }       
    }
}

fn generate_board(board: &[&str; 9]) {
    println!("{} | {} | {} ", board[0], board[1], board[2]);
    println!("{} | {} | {} ", board[3], board[4], board[5]);
    println!("{} | {} | {} ", board[6], board[7], board[8]);
}

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
        Ok(num) if num >= 1 && num <= 9 => Ok(num),
        _ => Err("Invalid input: Please enter a number between 1 and 9."),
    }

}


fn clear_screen() {
    print!("\x1B[2J\x1B[H"); // ANSI escape codes to clear screen and move cursor to beginning...
    io::stdout().flush().expect("Failed to flush stdout");
}

pub fn play_game(board: &mut [&str; 9]) {
    clear_screen();
    let mut player = GamePiece::X;
    loop {
        let x = get_user_input(&board, &player);
        match x {
            Ok(num) => {
                clear_screen();
                board[num - 1] = player.as_str();
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
