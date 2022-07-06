use crate::game_board::board::Board;

/// Checks if the current position is a repetition of a previous position
///
/// # Arguments
///
/// * `pos`: The current position
///
/// returns: bool

#[inline(always)]
pub fn is_repetition(pos: &Board) -> bool {
    for i in pos.history_ply - pos.fifty_move..pos.history_ply - 1 { //Only have to start looking after most recent capture
        if pos.pos_key == pos.history[i as usize].pos_key {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::alphabeta::search::is_repetition;
    use crate::game_board::board::Board;
    use crate::moves::gamemove::GameMove;
    use crate::utils::square_utils::fr2sq;
    use crate::constants::squares::{*};

    #[test]
    fn test_is_not_repetition() {
        let mut board = Board::new();
        unsafe{board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");}
        board.update_material_list();
        board.make_move(GameMove::new(fr2sq(FILE_B, RANK_1), fr2sq(FILE_C, RANK_3), 0, 0, 0));
        board.make_move(GameMove::new(fr2sq(FILE_B, RANK_8), fr2sq(FILE_C, RANK_6), 0, 0, 0));
        board.make_move(GameMove::new(fr2sq(FILE_E, RANK_2), fr2sq(FILE_E, RANK_3), 0, 0, 0));
        assert_eq!(is_repetition(&board), false);
    }

    #[test]
    fn test_is_repetition() {
        let mut board = Board::new();
        unsafe{board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");}
        board.update_material_list();
        board.make_move(GameMove::new(fr2sq(FILE_B, RANK_1), fr2sq(FILE_C, RANK_3), 0, 0, 0));
        board.make_move(GameMove::new(fr2sq(FILE_B, RANK_8), fr2sq(FILE_C, RANK_6), 0, 0, 0));
        board.make_move(GameMove::new(fr2sq(FILE_G, RANK_1), fr2sq(FILE_H, RANK_3), 0, 0, 0));
        board.make_move(GameMove::new(fr2sq(FILE_G, RANK_8), fr2sq(FILE_H, RANK_6), 0, 0, 0));
        board.make_move(GameMove::new(fr2sq(FILE_H, RANK_3), fr2sq(FILE_G, RANK_1), 0, 0, 0));
        board.make_move(GameMove::new(fr2sq(FILE_H, RANK_6), fr2sq(FILE_G, RANK_8), 0, 0, 0));
        assert_eq!(is_repetition(&board), true);
    }
}