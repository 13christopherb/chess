use crate::Board;
use crate::constants::pieces::{BIG_PIECE, BOTH, EMPTY, MAJOR_PIECE, PIECE_COLOR, VALUE};
use crate::moves::validate::is_sq_on_board;

const CASTLE_PERM:[u8; 120] = [
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 13, 15, 15, 15, 12, 15, 15, 14, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15,  7, 15, 15, 15,  3, 15, 15, 11, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15
]; // For bitwise and with castle perm because castle perm is kept as 4 bits

/// Removes a piece from the board state
///
/// # Arguments
///
/// * `pos`: The board state
/// * `sq`: The square the piece is sitting on. This should be a 120 square board number
///
/// returns: ()
#[inline]
fn clear_piece(pos:&mut Board, sq:u8) {
    let pce = pos.pieces[sq as usize] as usize;
    let col = PIECE_COLOR[pce as usize] as usize;
    let mut t_pce_num = -1;

    pos.hash_piece(pos.pieces[sq as usize], sq);

    pos.pieces[sq as usize] = EMPTY;
    pos.material[col] -= VALUE[pce];

    if BIG_PIECE[pce] {
        pos.num_big_pieces[col] -= 1;
        if MAJOR_PIECE[pce] {
            pos.num_major_pieces[col] -= 1;
        } else {
            pos.num_minor_pieces[col] -= 1;
        }
    } else {
        pos.bitboards[col].clear_bit(pos.sq64(sq));
        pos.bitboards[BOTH as usize].clear_bit(pos.sq64(sq));
    }

    for i in 0..pos.num_pieces[pce] as usize {
        if pos.piece_list[pce][i] == sq {
            t_pce_num = i as i16;
            break;
        }
    }

    if t_pce_num < 0 {
        panic!("Didn't find the right piece at the square");
    }

    pos.num_pieces[pce] -= 1;
    pos.piece_list[pce][t_pce_num as usize] = pos.piece_list[pce][pos.num_pieces[pce] as usize]; // Replace the removed piece with the last piece in the list, after decrementing the max index
}

#[cfg(test)]
mod test {
    use crate::Board;
    use crate::constants::pieces::{BLACK, WHITE, WR};
    use crate::moves::makemove::clear_piece;

    #[test]
    fn test_clear_piece_rook() {
        let fen = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        board.update_material_list();
        let before = board.clone();
        let sq = board.sq120(0);
        clear_piece(&mut board, sq);
        assert_eq!(before.num_major_pieces[WHITE as usize] - 1, board.num_major_pieces[WHITE as usize], "Did not decrement number of major pieces");
        assert_eq!(before.num_big_pieces[WHITE as usize] - 1, board.num_big_pieces[WHITE as usize], "Did not decrement number of big pieces");
        assert_eq!(before.num_pieces[WR as usize] - 1, board.num_pieces[WR as usize], "Did not decrement number of rooks");
        assert_eq!(before.num_major_pieces[BLACK as usize], board.num_major_pieces[BLACK as usize], "Decremented number of major pieces for wrong color");
        assert_eq!(before.num_minor_pieces[WHITE as usize], board.num_minor_pieces[WHITE as usize], "Decremented wrong category of piece");
        assert_eq!(before.bitboards[WHITE as usize].board, board.bitboards[WHITE as usize].board, "Changed bitboard when piece wasn't a pawn");
        assert_eq!(before.piece_list[WR as usize][1], board.piece_list[WR as usize][0], "Did not shorten piece list correctly");
    }

    //TODO: Test clear piece with pawns
}