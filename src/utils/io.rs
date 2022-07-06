use std::error::Error;
use std::{fmt, io};
use std::str::Chars;
use crate::game_board::board::Board;
use crate::constants::pieces::{*};
use crate::moves::gamemove::GameMove;
use crate::moves::movegen::generate_all_moves;
use crate::moves::validate::is_sq_on_board;
use crate::utils::square_utils::fr2sq;

#[derive(Debug, Clone)]
pub struct ParseMoveError;

impl fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid move string")
    }
}

/// Checks if a move input string is correctly formatted
///
/// # Arguments
///
/// * `text`: move string in form of a3b6
///
/// returns: bool
///
#[inline(always)]
fn text_is_correct_format(text: &str) -> bool {
    let mut itr = text.chars();
    match itr.next().unwrap_or('z') {
        'a'..='h' => (),
        _ => return false,
    }
    match itr.next().unwrap_or('9') {
        '1'..='8' => (),
        _ => return false,
    }
    match itr.next().unwrap_or('z') {
        'a'..='h' => (),
        _ => return false,
    }
    match itr.next().unwrap_or('9') {
        '1'..='8' => (),
        _ => return false,
    }
    true
}

#[inline(always)]
fn get_square(itr: &mut Chars<'_>) -> Result<u8, ParseMoveError> {
    let file = itr.next().ok_or_else(|| ParseMoveError)? as u8 - 'a' as u8;
    let rank = itr.next().ok_or_else(|| ParseMoveError)? as u8 - '1' as u8;
    Ok(fr2sq(file, rank))
}

pub fn validate_move(pos: &Board, text: &str) -> Result<(u8, u8, char), ParseMoveError> {
    if !text_is_correct_format(text) { return Err(ParseMoveError); }
    let mut itr = text.chars();
    let from =get_square(&mut itr)?;
    if PIECE_COLOR[pos.pieces[from as usize] as usize] != pos.side { return Err(ParseMoveError); }
    let to = get_square(&mut itr)?;
    if !is_sq_on_board(from) || !is_sq_on_board(to) { return Err(ParseMoveError); }
    let prom_char = text.chars().nth(4).unwrap_or('-');
    Ok((from, to, prom_char))
}

pub fn parse_move(pos: &Board, from:u8, to:u8, prom_char:char) -> Option<GameMove> {
    let mut move_list:Vec<GameMove> = Vec::new();
    generate_all_moves(pos, &mut move_list);
    let mut prom_piece:u8;
    for mov in move_list {
        if mov.origin() == from && mov.destination() == to {
            prom_piece = mov.promoted_piece();
            if prom_piece != EMPTY {
                match prom_piece {
                    WR | BR => if prom_char == 'r' {return Some(mov);},
                    WB | BB => if prom_char == 'b' {return Some(mov);},
                    WN | BN => if prom_char == 'n' {return Some(mov); },
                    WQ | BQ => if prom_char == 'q' {return Some(mov); },
                    _ => continue
                }
            }
            return Some(mov);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::game_board::board::Board;
    use crate::moves::gamemove::GameMove;
    use crate::constants::squares::{*};
    use crate::utils::io::parse_move;
    use crate::utils::square_utils::fr2sq;

    #[test]
    fn test_parse_move() {
        let from = fr2sq(FILE_A, RANK_2);
        let to = fr2sq(FILE_A, RANK_4);
        let prom_char = '-';
        let expected_move = GameMove::new(fr2sq(FILE_A, RANK_2), fr2sq(FILE_A, RANK_4), 0, 0, 0x80000);
        let mut board = Board::new();
        unsafe {board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")};
        board.update_material_list();
        let res = parse_move(&board, from, to, prom_char);
        let mut  mov:GameMove = GameMove::new(0, 0, 0, 0, 0);
        match res {
            Some(m) => mov = m,
            None => assert!(false, "Did not find it as a correct move"),
        }
        assert_eq!(mov.origin(), expected_move.origin(), "Did not give correct starting square");
        assert_eq!(mov.destination(), expected_move.destination(), "Did not give correct destination square");
        assert_eq!(mov.is_pawn_start(), expected_move.is_pawn_start(), "Did not set pawn start flag");
    }
}