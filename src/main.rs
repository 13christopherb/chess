mod bitboard;
mod bit_operations;
mod board;
mod constants;
mod hashkeys;
mod utils;

use crate::board::Board;


fn main() {
    let start: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let white_pawn_bb: u64 = 0x0000_0000_0000_FF00;

    let mut board = Board::new();
    unsafe { board.parse_fen(start) };
    print!("{}", board);
}
