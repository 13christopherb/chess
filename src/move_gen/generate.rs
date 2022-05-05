use crate::{Board, check_board};
use crate::constants::{pieces, ranks, squares, ranks::RANK_SQUARES};
use crate::constants::squares::is_sq_on_board;
use crate::move_gen::moves::{GameMove, MFLAG_EP, MFLAG_PS};
use crate::utils::square_utils::{fr2sq, init_file_rank_arrays};

const MAX_POSITION_MOVES:u32 = 256;

#[inline(always)]
fn add_quiet_move(pos:&Board, mve:GameMove, list:&mut Vec<GameMove>) {
    list.push(mve);
}

#[inline(always)]
fn add_capture_move(pos:&Board, mve: GameMove, list:&mut Vec<GameMove>) {
    list.push(mve);
}

#[inline(always)]
fn add_wp_capture_move(pos:&Board, from:u8, to:u8, cap:u8, list:&mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == ranks::RANK_7 {
        add_capture_move(pos,GameMove::new(from, to, cap, pieces::WQ, 0), list);
        add_capture_move(pos,GameMove::new(from, to, cap, pieces::WR, 0), list);
        add_capture_move(pos,GameMove::new(from, to, cap, pieces::WB, 0), list);
        add_capture_move(pos,GameMove::new(from, to, cap, pieces::WN, 0), list);
    } else {
        add_capture_move(pos,GameMove::new(from, to, cap, pieces::EMPTY, 0), list);
    }
}

#[inline(always)]
fn add_wp_move(pos:&Board, from:u8, to:u8, list:&mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == ranks::RANK_7 {
        add_quiet_move(pos,GameMove::new(from, to, pieces::EMPTY, pieces::WQ, 0), list);
        add_quiet_move(pos,GameMove::new(from, to, pieces::EMPTY, pieces::WR, 0), list);
        add_quiet_move(pos,GameMove::new(from, to, pieces::EMPTY, pieces::WB, 0), list);
        add_quiet_move(pos,GameMove::new(from, to, pieces::EMPTY, pieces::WN, 0), list);
    } else {
        add_quiet_move(pos,GameMove::new(from, to, pieces::EMPTY, pieces::EMPTY, 0), list);
    }
}

fn add_enpassant_move(mve:u32, list:&mut Vec<GameMove>) {
    list.push(GameMove{move_int: mve, score: 0})
}

fn generate_all_moves(pos:&Board, list:&mut Vec<GameMove>) {
    assert!(check_board(pos));

    let mut sq;
    let mut sqi:usize;
    if pos.side == pieces::WHITE {
        for pce_num in 0..pos.pieces[pieces::WP as usize] as usize {
            sq = pos.piece_list[pieces::WP as usize][pce_num];
            sqi = sq as usize;
            if pos.pieces[sqi + 10] == pieces::EMPTY {
                add_wp_move(pos, sq, sq + 10, list);
                // If pawn can move two squares
                if RANK_SQUARES[sqi] == ranks::RANK_2 && pos.pieces[sqi + 20] == pieces::EMPTY {
                    add_quiet_move(pos, GameMove::new(sq,
                                                           sq + 10,
                                                           pieces::EMPTY,
                                                           pieces::EMPTY,
                                                           MFLAG_PS),
                                        list)
                }

                if is_sq_on_board(sq) && pieces::is_black(pos.pieces[sqi + 9]) {
                    add_wp_capture_move(pos, sq, sq + 9, pos.pieces[sqi + 9], list);
                }
                if is_sq_on_board(sq) && pieces::is_black(pos.pieces[sqi + 11]) {
                    add_wp_capture_move(pos, sq, sq + 9, pos.pieces[sqi + 11], list);
                }

                if sq + 9 == pos.en_passant {
                    add_capture_move(pos, GameMove::new(sq,
                                                        sq + 9,
                                                        pieces::EMPTY,
                                                        pieces::EMPTY,
                                                        MFLAG_EP), list);
                }
                if sq + 11 == pos.en_passant {
                    add_capture_move(pos, GameMove::new(sq,
                                                        sq + 11,
                                                        pieces::EMPTY,
                                                        pieces::EMPTY,
                                                        MFLAG_EP), list);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Board;
    use crate::constants::{files, pieces, ranks};
    use crate::move_gen::generate::add_wp_capture_move;
    use crate::move_gen::moves::GameMove;
    use crate::utils::square_utils::fr2sq;

    #[test]
    fn test_white_pawn_capture_promote() {
        let fen = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P2P/RNBQKBNR w KQkq e6 0 10";
        let mut board = Board::new();
        unsafe{ board.parse_fen(fen); }
        let mut move_list:Vec<GameMove> = Vec::new();
        add_wp_capture_move(&board, fr2sq(files::FILE_G, ranks::RANK_7),
                            fr2sq(files::FILE_H, ranks::RANK_8),
                            pieces::BR, &mut move_list);
        assert_eq!(move_list.len(), 4, "Did not generate correct number of moves");
        assert_eq!(move_list[0].move_int, 0x52b157, "Did not generate gxh8=Q?"); //gxh8=Q?
        assert_eq!(move_list[3].move_int, 0x22b157, "Did not generate gxh8=N?"); //gxh8=N?
    }

    fn test_white_pawn_capture() {
        let fen = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P2P/RNBQKBNR w KQkq e6 0 10";
        let mut board = Board::new();
        unsafe{ board.parse_fen(fen); }
        let mut move_list:Vec<GameMove> = Vec::new();
        add_wp_capture_move(&board, fr2sq(files::FILE_B, ranks::RANK_4),
                            fr2sq(files::FILE_C, ranks::RANK_5),
                            pieces::BP, &mut move_list);
        assert_eq!(move_list.len(), 1, "Did not generate correct number of moves");
        assert_eq!(move_list[0].move_int, 0x1d119, "Did not generate bxc5"); //bxc5
    }

}


