mod game_board;
mod constants;
mod utils;
mod move_gen;

use crate::game_board::board::{Board, check_board};

fn testing(test:&Board) {
    let mut x = 2;
    x += 1;
}

fn main() {
    let start: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let white_pawn_bb: u64 = 0x0000_0000_0000_FF00;

    let mut board = Board::new();
    unsafe { board.parse_fen(start) };
    testing(&board);
    board.update_material_list();
    assert!(check_board(&board));
    print!("{}", board);
}
