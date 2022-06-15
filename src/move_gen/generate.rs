use crate::constants::{castling::*, pieces::*, ranks, ranks::RANK_SQUARES, sqs::*};
use crate::move_gen::attack::is_square_attacked;
use crate::move_gen::moves::{GameMove, MFLAG_CA, MFLAG_EP, MFLAG_PS};
use crate::utils::square_utils::{fr2sq, init_file_rank_arrays};
use crate::{check_board, Board};

const MAX_POSITION_MOVES: u32 = 256;

#[inline(always)]
fn add_quiet_move(pos: &Board, mve: GameMove, list: &mut Vec<GameMove>) {
    list.push(mve);
}

#[inline(always)]
fn add_capture_move(pos: &Board, mve: GameMove, list: &mut Vec<GameMove>) {
    list.push(mve);
}

#[inline(always)]
fn add_wp_capture_move(pos: &Board, from: u8, to: u8, cap: u8, list: &mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == ranks::RANK_7 {
        add_capture_move(pos, GameMove::new(from, to, cap, WQ, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, WR, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, WB, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, WN, 0), list);
    } else {
        add_capture_move(pos, GameMove::new(from, to, cap, EMPTY, 0), list);
    }
}

#[inline(always)]
fn add_wp_move(pos: &Board, from: u8, to: u8, list: &mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == ranks::RANK_7 {
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, WQ, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, WR, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, WB, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, WN, 0), list);
    } else {
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, EMPTY, 0), list);
    }
}

#[inline(always)]
fn add_bp_capture_move(pos: &Board, from: u8, to: u8, cap: u8, list: &mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == ranks::RANK_2 {
        add_capture_move(pos, GameMove::new(from, to, cap, BQ, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, BR, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, BB, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, BN, 0), list);
    } else {
        add_capture_move(pos, GameMove::new(from, to, cap, EMPTY, 0), list);
    }
}

#[inline(always)]
fn add_bp_move(pos: &Board, from: u8, to: u8, list: &mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == ranks::RANK_2 {
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, BQ, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, BR, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, BB, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, BN, 0), list);
    } else {
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, EMPTY, 0), list);
    }
}

/// Generates all valid moves for white pawns
///
/// # Arguments
///
/// * `pos`: game board with game's current state
/// * `list`: a vec that the moves will be added to
///
/// returns: ()
///

#[inline(always)]
fn generate_wp_moves(pos: &Board, list: &mut Vec<GameMove>) {
    let mut sq;
    let mut sqi: usize;
    for pce_num in 0..pos.num_pieces[WP as usize] as usize {
        sq = pos.piece_list[WP as usize][pce_num];
        sqi = sq as usize;
        if pos.pieces[sqi + 10] == EMPTY {
            add_wp_move(pos, sq, sq + 10, list);
            // If pawn can move two squares
            if RANK_SQUARES[sqi] == ranks::RANK_2 && pos.pieces[sqi + 20] == EMPTY {
                add_quiet_move(
                    pos,
                    GameMove::new(sq, sq + 20, EMPTY, EMPTY, MFLAG_PS),
                    list,
                )
            }
        }

        if is_sq_on_board(sq as i32) && is_black(pos.pieces[sqi + 9]) {
            add_wp_capture_move(pos, sq, sq + 9, pos.pieces[sqi + 9], list);
        }
        if is_sq_on_board(sq as i32) && is_black(pos.pieces[sqi + 11]) {
            add_wp_capture_move(pos, sq, sq + 11, pos.pieces[sqi + 11], list);
        }

        if sq + 9 == pos.en_passant {
            add_capture_move(pos, GameMove::new(sq, sq + 9, EMPTY, EMPTY, MFLAG_EP), list);
        }
        if sq + 11 == pos.en_passant {
            add_capture_move(
                pos,
                GameMove::new(sq, sq + 11, EMPTY, EMPTY, MFLAG_EP),
                list,
            );
        }
    }
}

/// Generates all valid moves for black pawns
///
/// # Arguments
///
/// * `pos`: game board with game's current state
/// * `list`: a vec that the moves will be added to
///
/// returns: ()
///

#[inline(always)]
fn generate_bp_moves(pos: &Board, list: &mut Vec<GameMove>) {
    let mut sq;
    let mut sqi: usize;
    for pce_num in 0..pos.num_pieces[BP as usize] as usize {
        sq = pos.piece_list[BP as usize][pce_num];
        sqi = sq as usize;
        if pos.pieces[sqi - 10] == EMPTY {
            add_bp_move(pos, sq, sq - 10, list);
            // If pawn can move two squares
            if RANK_SQUARES[sqi] == ranks::RANK_7 && pos.pieces[sqi - 20] == EMPTY {
                add_quiet_move(
                    pos,
                    GameMove::new(sq, sq - 20, EMPTY, EMPTY, MFLAG_PS),
                    list,
                )
            }
        }

        if is_sq_on_board(sq as i32) && is_white(pos.pieces[sqi - 9]) {
            add_bp_capture_move(pos, sq, sq - 9, pos.pieces[sqi - 9], list);
        }
        if is_sq_on_board(sq as i32) && is_white(pos.pieces[sqi - 11]) {
            add_bp_capture_move(pos, sq, sq - 11, pos.pieces[sqi - 11], list);
        }

        if sq - 9 == pos.en_passant {
            add_capture_move(pos, GameMove::new(sq, sq - 9, EMPTY, EMPTY, MFLAG_EP), list);
        }
        if sq - 11 == pos.en_passant {
            add_capture_move(
                pos,
                GameMove::new(sq, sq - 11, EMPTY, EMPTY, MFLAG_EP),
                list,
            );
        }
    }
}

/// Generates all possible moves for rooks, bishops, and queens for the specified side and
/// board state
///
/// # Arguments
///
/// * `pos`: The board state
/// * `list`: The vec all of the moves will be added to
/// * `side`: The side to generate moves for (0 = white, 1 = black)
///
/// returns: ()
///

#[inline(always)]
fn generate_sliding_moves(pos: &Board, list: &mut Vec<GameMove>, side: u8) {
    let mut piece_idx = LOOP_SLIDE_INDEX[side as usize] as usize; // Start at different points in loop array depending on color
    let mut piece = LOOP_SLIDE[piece_idx] as usize;

    while piece != 0 {
        for i in 0..pos.num_pieces[piece as usize] as usize {
            let sq = pos.piece_list[piece][i] as i32;

            for j in 0..NUM_DIR[piece] {
                let dir = PIECE_DIR[piece][j];
                let mut t_sq = sq + dir;

                while is_sq_on_board(t_sq) {
                    let t_sqi = t_sq as usize;
                    if pos.pieces[t_sqi] != EMPTY {
                        if PIECE_COLOR[pos.pieces[t_sqi] as usize] == side ^ 1 {
                            add_capture_move(
                                pos,
                                GameMove::new(sq as u8, t_sq as u8, pos.pieces[t_sqi], EMPTY, 0),
                                list,
                            );
                        }
                        break;
                    }
                    add_quiet_move(
                        pos,
                        GameMove::new(sq as u8, t_sq as u8, EMPTY, EMPTY, 0),
                        list,
                    );
                    t_sq += dir;
                }
            }
        }
        piece_idx += 1;
        piece = LOOP_SLIDE[piece_idx] as usize;
    }
}

/// Generates all moves for knights and non-castling king moves for the specified side and
/// board state
///
/// # Arguments
///
/// * `pos`: The board state
/// * `list`: The vec all moves will be added to
/// * `side`: The side to generate moves for (0 = white, 1 = black)
///
/// returns: ()
#[inline(always)]
fn generate_nonsliding_moves(pos: &Board, list: &mut Vec<GameMove>, side: u8) {
    let mut piece_idx = LOOP_NONSLIDE_INDEX[side as usize] as usize;
    let mut piece = LOOP_NONSLIDE[piece_idx] as usize;

    while piece != 0 {
        for i in 0..pos.num_pieces[piece as usize] as usize {
            let sq = pos.piece_list[piece][i] as i32;

            for j in 0..NUM_DIR[piece] {
                let dir = PIECE_DIR[piece][j];
                let t_sq = sq + dir;
                if !is_sq_on_board(t_sq) {
                    continue;
                }
                let t_sq = t_sq as usize;

                if pos.pieces[t_sq] != EMPTY {
                    if PIECE_COLOR[pos.pieces[t_sq] as usize] == side ^ 1 {
                        add_capture_move(
                            pos,
                            GameMove::new(sq as u8, t_sq as u8, pos.pieces[t_sq], EMPTY, 0),
                            list,
                        );
                    }
                    continue;
                }
                add_quiet_move(
                    pos,
                    GameMove::new(sq as u8, t_sq as u8, EMPTY, EMPTY, 0),
                    list,
                );
            }
        }
        piece_idx += 1;
        piece = LOOP_NONSLIDE[piece_idx] as usize;
    }
}

fn generate_all_moves(pos: &Board, list: &mut Vec<GameMove>) {
    assert!(check_board(pos));

    if pos.side == WHITE {
        generate_wp_moves(pos, list); // Pawns have a lot of special rules for movement, best to write specific functions

        if pos.castle_perm & WK_CASTLE != 0 { // White king castling
            if pos.pieces[F1] == EMPTY && pos.pieces[G1] == EMPTY {
                if !is_square_attacked(E1 as u8, BLACK, &pos.pieces)
                    && !is_square_attacked(F1 as u8, BLACK, &pos.pieces)
                {
                    add_quiet_move(
                        pos,
                        GameMove::new(E1 as u8, G1 as u8, EMPTY, EMPTY, MFLAG_CA),
                        list,
                    );
                }
            }
        }

        if pos.castle_perm & WQ_CASTLE != 0 { // White queen castling
            if pos.pieces[D1] == EMPTY && pos.pieces[C1] == EMPTY && pos.pieces[B1] == EMPTY {
                if !is_square_attacked(D1 as u8, BLACK, &pos.pieces)
                    && !is_square_attacked(C1 as u8, BLACK, &pos.pieces)
                    && !is_square_attacked(B1 as u8, BLACK, &pos.pieces)
                {
                    add_quiet_move(
                        pos,
                        GameMove::new(E1 as u8, C1 as u8, EMPTY, EMPTY, MFLAG_CA),
                        list,
                    );
                }
            }
        }

        generate_sliding_moves(pos, list, WHITE);
        generate_nonsliding_moves(pos, list, WHITE);
    } else {
        generate_bp_moves(pos, list);

        if pos.castle_perm & BK_CASTLE != 0 {         // Black king castling
            if pos.pieces[F8] == EMPTY && pos.pieces[G8] == EMPTY {
                if !is_square_attacked(E8 as u8, WHITE, &pos.pieces)
                    && !is_square_attacked(F8 as u8, WHITE, &pos.pieces)
                {
                    add_quiet_move(
                        pos,
                        GameMove::new(E8 as u8, G8 as u8, EMPTY, EMPTY, MFLAG_CA),
                        list,
                    );
                }
            }
        }

        if pos.castle_perm & BQ_CASTLE != 0 {   // Black queen castling
            if pos.pieces[D8] == EMPTY && pos.pieces[C8] == EMPTY && pos.pieces[B8] == EMPTY {
                if !is_square_attacked(D8 as u8, WHITE, &pos.pieces)
                    && !is_square_attacked(C8 as u8, WHITE, &pos.pieces)
                    && !is_square_attacked(B8 as u8, WHITE, &pos.pieces)
                {
                    add_quiet_move(
                        pos,
                        GameMove::new(E8 as u8, C8 as u8, EMPTY, EMPTY, MFLAG_CA),
                        list,
                    );
                }
            }
        }
        generate_sliding_moves(pos, list, BLACK);
        generate_nonsliding_moves(pos, list, BLACK);
    }
}

#[cfg(test)]
mod test {
    use crate::constants::files::FILE_SQUARES;
    use crate::constants::{files, pieces, ranks};
    use crate::move_gen::generate::*;
    use crate::move_gen::moves::GameMove;
    use crate::utils::square_utils::fr2sq;
    use crate::Board;
    use std::env;

    #[test]
    fn test_white_pawn_capture_promote() {
        let fen = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        let mut move_list: Vec<GameMove> = Vec::new();
        add_wp_capture_move(
            &board,
            fr2sq(files::FILE_G, ranks::RANK_7),
            fr2sq(files::FILE_H, ranks::RANK_8),
            pieces::BR,
            &mut move_list,
        );
        assert_eq!(
            move_list.len(),
            4,
            "Did not generate correct number of moves"
        );
        assert_eq!(move_list[0].move_int, 0x52b157, "Did not generate gxh8=Q?"); //gxh8=Q?
        assert_eq!(move_list[3].move_int, 0x22b157, "Did not generate gxh8=N?");
        //gxh8=N?
    }

    fn test_white_pawn_capture() {
        let fen = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        let mut move_list: Vec<GameMove> = Vec::new();
        add_wp_capture_move(
            &board,
            fr2sq(files::FILE_B, ranks::RANK_4),
            fr2sq(files::FILE_C, ranks::RANK_5),
            pieces::BP,
            &mut move_list,
        );
        assert_eq!(
            move_list.len(),
            1,
            "Did not generate correct number of moves"
        );
        assert_eq!(move_list[0].move_int, 0x1d119, "Did not generate bxc5"); //bxc5
    }

    #[test]
    fn test_black_pawn_capture_promote() {
        let fen = "rnbqkbnr/ppp1p2p/3p4/3p4/2P1Pp2/8/PP1P1PpP/RNBQKBNR b KQkq e3 0 10";
        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        let mut move_list: Vec<GameMove> = Vec::new();
        add_bp_capture_move(
            &board,
            fr2sq(files::FILE_G, ranks::RANK_2),
            fr2sq(files::FILE_H, ranks::RANK_1),
            pieces::WR,
            &mut move_list,
        );

        assert_eq!(
            move_list.len(),
            4,
            "Did not generate correct number of moves"
        );
        assert_eq!(move_list[0].move_int, 0xb10e25, "Did not generate gxh1=Q?"); //gxh1=Q?
        assert_eq!(move_list[3].move_int, 0x810e25, "Did not generate gxh1=N?");
        //gxh1=N?
    }

    #[test]
    fn test_black_pawn_capture() {
        let fen = "rnbqkbnr/ppp1p2p/3p4/3p4/2P1Pp2/8/PP1P1PpP/RNBQKBNR b KQkq e3 0 10";
        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        let mut move_list: Vec<GameMove> = Vec::new();
        add_bp_capture_move(
            &board,
            fr2sq(files::FILE_D, ranks::RANK_5),
            fr2sq(files::FILE_C, ranks::RANK_4),
            pieces::WP,
            &mut move_list,
        );
        assert_eq!(
            move_list.len(),
            1,
            "Did not generate correct number of moves"
        );
        assert_eq!(move_list[0].move_int, 0x5ac0, "Did not generate cxb4"); //dxc4
    }

    #[test]
    fn test_wp_movegen() {
        let fen = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P2P/RNBQKBNR w KQkq e6 0 10";
        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        board.update_material_list();
        let mut move_list: Vec<GameMove> = Vec::new();
        generate_wp_moves(&board, &mut move_list);
        assert_eq!(
            move_list.len(),
            23,
            "Did not generate correct number of white pawn moves"
        );
    }

    #[test]
    fn test_bp_movegen() {
        let fen = "rnbqkbnr/ppp1p2p/3p4/3p4/2P1Pp2/8/PP1P1PpP/RNBQKBNR b KQkq e3 0 10";
        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        board.update_material_list();
        let mut move_list: Vec<GameMove> = Vec::new();
        generate_bp_moves(&board, &mut move_list);
        assert_eq!(
            move_list.len(),
            23,
            "Did not generate correct number of white pawn moves"
        );
    }

    #[test]
    fn test_all_movegen_white() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let sample_moves = vec![
            GameMove {
                move_int: 140846,
                score: 0,
            },
            GameMove {
                move_int: 16780697,
                score: 0,
            },
            GameMove {
                move_int: 531621,
                score: 0,
            },
            GameMove {
                move_int: 5279,
                score: 0,
            },
            GameMove {
                move_int: 9536,
                score: 0,
            },
            GameMove {
                move_int: 6819,
                score: 0,
            },
            GameMove {
                move_int: 156579,
                score: 0,
            },
            GameMove {
                move_int: 3356,
                score: 0,
            },
            GameMove {
                move_int: 3115,
                score: 0,
            },
        ];

        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        board.update_material_list();
        let mut move_list: Vec<GameMove> = Vec::new();
        generate_all_moves(&board, &mut move_list);

        assert_eq!(
            move_list.len(),
            48,
            "Did not generate correct number of moves for white"
        );

        assert!(sample_moves.iter().all(|item| move_list.contains(item)));
    }

    #[test]
    fn test_all_movegen_black() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1";

        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        board.update_material_list();
        let mut move_list: Vec<GameMove> = Vec::new();
        generate_all_moves(&board, &mut move_list);

        assert_eq!(
            move_list.len(),
            43,
            "Did not generate correct number of moves for black"
        );
    }
}
