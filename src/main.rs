fn main() {
    let mut board: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    board[0] = "X";
    board[1] = "O";
    board[2] = "X";
    board[0] = "O";

    // TODO: Generate the play_game function...

    generate_board(board);

}

fn generate_board(board: [&str; 9]) {
    println!("{} | {} | {} ", board[0], board[1], board[2]);
    println!("{} | {} | {} ", board[3], board[4], board[5]);
    println!("{} | {} | {} ", board[6], board[7], board[8]);
}
