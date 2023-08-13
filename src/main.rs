use tictactoe::play_game;

fn main() {
    let mut board: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    play_game(&mut board);
}