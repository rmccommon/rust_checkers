mod piece;
mod game_board;



fn main() {
    let board1 = game_board::Board::new();
    let piece1 = piece::Piece::new(0, 0);
    println!("{}", board1);
}
