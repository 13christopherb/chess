mod bitboard;
mod bit_operations;
mod board;
mod constants;

use crate::bitboard::bitboard::BitBoard;
use crate::board::Board;


fn main() {
    let initial_bits:u64 =  0b00100001_00000001_00000001_00010001_00000001_00000001_00000001_00000001;
    let mut board:BitBoard = BitBoard::new(initial_bits);
    print!("{}", board);
    board.pop_bit();
    print!("{}", board);
}
