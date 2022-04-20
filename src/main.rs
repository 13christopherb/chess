mod bitboard;
mod bit_operations;
mod board;

use crate::bitboard::BitBoard;
use crate::board::Board;


fn main() {
    let initial_bits:u64 =  0b00100001_00000001_00000001_00010001_00000001_00000001_00000001_00000001;
    let board:BitBoard = BitBoard(initial_bits);
    print!("{}", board);
}
