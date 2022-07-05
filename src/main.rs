use std::io;
use std::io::prelude::*;
use chess::game_board::board::Board;
use chess::utils::io::parse_move;

fn main() {
    let mut board = Board::new();
    unsafe {board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");}
    board.update_material_list();
    let stdin = io::stdin();
    println!("{}", board);
    println!("Please enter a move:");
    for line in stdin.lock().lines() {
        let res = parse_move(&board, line.as_ref().unwrap().as_str());
        let mut mov;
        match res {
            Some(m) => mov = m,
            None => panic!("Invalid move"),
        }
        board.make_move(mov);
        println!("{}", board);
        println!("Enter next move");
    }
}