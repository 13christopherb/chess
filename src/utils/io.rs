use crate::game_board::board::Board;
use crate::constants::pieces::{*};
use crate::moves::gamemove::GameMove;
use crate::moves::movegen::generate_all_moves;
use crate::moves::validate::is_sq_on_board;
use crate::utils::square_utils::fr2sq;

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
    match itr.next().unwrap() {
        'a'..='h' => (),
        _ => return false,
    }
    match itr.next().unwrap() {
        '1'..='8' => (),
        _ => return false,
    }
    match itr.next().unwrap() {
        'a'..='h' => (),
        _ => return false,
    }
    match itr.next().unwrap() {
        '1'..='8' => (),
        _ => return false,
    }
    true
}

pub fn parse_move(pos: &Board, text: &str) -> Option<GameMove> {
    if !text_is_correct_format(text) { return None; }
    let mut itr = text.chars();
    let from = fr2sq(itr.next().unwrap() as u8 - 'a' as u8, itr.next().unwrap() as u8 - '1' as u8);
    let to = fr2sq(itr.next().unwrap() as u8 - 'a' as u8, itr.next().unwrap() as u8 - '1' as u8);

    if !is_sq_on_board(from) || !is_sq_on_board(to) { panic!("Incorrect square"); }
    let mut move_list:Vec<GameMove> = Vec::new();
    generate_all_moves(pos, &mut move_list);
    let mut prom_piece:u8;
    for mov in move_list {
        if mov.origin() == from && mov.destination() == to {
            prom_piece = mov.promoted_piece();
            if prom_piece != EMPTY {
                let prom_char = text.chars().nth(4).unwrap();
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
        let text = "a2a4";
        let expected_move = GameMove::new(fr2sq(FILE_A, RANK_2), fr2sq(FILE_A, RANK_4), 0, 0, 0x80000);
        let mut board = Board::new();
        unsafe {board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")};
        board.update_material_list();
        let res = parse_move(&board, text);
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