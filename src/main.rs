mod game_board;
mod constants;
mod utils;
mod move_gen;

use crate::game_board::board::{Board, check_board};


fn main() {
    let start: &str = "rnbqkbnr/ppp1p2p/3p4/3p4/2P1Pp2/8/PP1P1PpP/RNBQKBNR b KQkq e3 0 10";
    let white_pawn_bb: u64 = 0x0000_0000_0000_FF00;

    let mut board = Board::new();
    unsafe { board.parse_fen(start) };
    board.update_material_list();
    for i in 0..board.num_pieces[7] {
        println!("{}", board.piece_list[7][i as usize]);
    }
}
