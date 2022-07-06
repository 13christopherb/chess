use std::io;
use std::io::prelude::*;
use chess::game_board::board::Board;
use chess::utils::io::{parse_move, validate_move};

fn main() {
    let mut board = Board::new();
    unsafe {board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");}
    board.update_material_list();
    let stdin = io::stdin();
    println!("{}", board);
    println!("Please enter a move:");
    let mut from:u8;
    let mut to:u8;
    let mut prom_char:char;
    for line in stdin.lock().lines() {
        let text = line.as_ref().expect("Problem reading from command line: {:?}").as_str();
        let (from, to, prom_char) = match validate_move(&board, text) {
            Ok(T) => T,
            Err(E) => {
                println!("Invalid move entered. Please try again:");
                continue;
            }
        };
        let mut mov = match parse_move(&board, from, to, prom_char) {
            Some(T) => T,
            None => {
                println!("Checkmate");
                break;
            }
        };

        board.make_move(mov);
        println!("{}", board);
        println!("Enter next move");
    }
}