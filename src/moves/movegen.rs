use crate::constants::{pieces::*, pieces, squares::*};
use crate::moves::gamemove::{GameMove, MFLAG_CA, MFLAG_EP, MFLAG_PS};
use crate::moves::validate::{*};
use crate::utils::square_utils::{fr2sq, init_file_rank_arrays};
use crate::game_board::board::{Board, check_board};

const MAX_POSITION_MOVES: u32 = 256;

pub fn sliding_piece_attacking(sq: u8, side: u8, pces: &[u8; 120]) -> bool {
    let mut piece = LOOP_SLIDE[LOOP_SLIDE_INDEX[side as usize]];
    let mut piece_s = piece as usize;
    while !is_queen(piece) { // Bishop and rook directions cover queen directions, so no need to check queen by itself
        for i in 0..NUM_DIR[piece_s] {
            let dir = PIECE_DIR[piece_s][i];

            let mut t_sq: u8 = sq.wrapping_add(dir as u8);

            while is_sq_on_board(t_sq) {
                let pce = pces[t_sq as usize];
                if pce != EMPTY {
                    if is_same_color(pce, side) && (pce == piece || is_queen(pce)) {
                        return true;
                    }
                    break;
                }
                t_sq = t_sq.wrapping_add(dir as u8);
            }
        }
        piece += 1;
        piece_s = piece as usize;
    }
    false
}


/// Determines if a given square is being attacked by a piece of the specified color
///
/// # Arguments
///
/// * `sq`: the square number (in 0-120 squares) that might be being attacked
/// * `side`: the color of piece to look for attacking (using pieces:: constants)
/// * `pces`: array slice containing all the pieces on the board
///
/// returns: bool true if the square is attacked by any piece of the specified color
///
/// # Examples
///
/// ```is_square_attacked(86, pieces::WHITE, &board.pieces)
///
/// ```
pub fn square_is_attacked(sq: u8, side: u8, pces: &[u8; 120]) -> bool {
    // Pawns
    if side == pieces::WHITE {
        if pces[(sq - 11) as usize] == pieces::WP || pces[(sq - 9) as usize] == pieces::WP {
            return true;
        }
    } else {
        if pces[(sq + 11) as usize] == pieces::BP || pces[(sq + 9) as usize] == pieces::BP {
            return true;
        }
    }
    // Knights
    for dir in PIECE_DIR[WN as usize] { // Color doesn't matter
        let pce = pces[(sq.wrapping_add(dir as u8)) as usize];
        if pce != OFFBOARD && is_knight(pce) && is_same_color(pce, side) {
            return true;
        }
    }

    if sliding_piece_attacking(sq, side, pces) { return true; }

    // Kings

    let t_sq = sq as i32;

    for dir in PIECE_DIR[WK as usize] {
        let pce = pces[(t_sq + dir) as usize];
        if pce == OFFBOARD { break; }
        if is_king(pce) && is_same_color(pce, side) {
            return true;
        }
    }
    false
}

#[inline]
fn squares_are_attacked(squares: &[usize], side: u8, pces: &[u8; 120]) -> bool {
    for sq in squares {
        if square_is_attacked(*sq as u8, side, pces) { return true; }
    }
    false
}

#[inline]
fn squares_are_empty(squares: &[usize], pces: &[u8; 120]) -> bool {
    for sq in squares {
        if pces[*sq] != EMPTY { return false; }
    }
    true
}

#[inline(always)]
fn add_quiet_move(pos: &Board, mve: GameMove, list: &mut Vec<GameMove>) {
    list.push(mve);
}

#[inline(always)]
fn add_capture_move(pos: &Board, mve: GameMove, list: &mut Vec<GameMove>) {
    list.push(mve);
}

fn add_wp_capture_move(pos: &Board, from: u8, to: u8, cap: u8, list: &mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == RANK_7 {
        add_capture_move(pos, GameMove::new(from, to, cap, WQ, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, WR, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, WB, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, WN, 0), list);
    } else {
        add_capture_move(pos, GameMove::new(from, to, cap, EMPTY, 0), list);
    }
}

fn add_wp_move(pos: &Board, from: u8, to: u8, list: &mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == RANK_7 {
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, WQ, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, WR, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, WB, 0), list);
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, WN, 0), list);
    } else {
        add_quiet_move(pos, GameMove::new(from, to, EMPTY, EMPTY, 0), list);
    }
}

fn add_bp_capture_move(pos: &Board, from: u8, to: u8, cap: u8, list: &mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == RANK_2 {
        add_capture_move(pos, GameMove::new(from, to, cap, BQ, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, BR, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, BB, 0), list);
        add_capture_move(pos, GameMove::new(from, to, cap, BN, 0), list);
    } else {
        add_capture_move(pos, GameMove::new(from, to, cap, EMPTY, 0), list);
    }
}

fn add_bp_move(pos: &Board, from: u8, to: u8, list: &mut Vec<GameMove>) {
    if RANK_SQUARES[from as usize] == RANK_2 {
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

fn generate_wp_moves(pos: &Board, list: &mut Vec<GameMove>) {
    let mut sq;
    let mut sqi: usize;
    for pce_num in 0..pos.num_pieces[WP as usize] as usize {
        sq = pos.piece_list[WP as usize][pce_num];
        sqi = sq as usize;
        if pos.pieces[sqi + 10] == EMPTY {
            add_wp_move(pos, sq, sq + 10, list);
            // If pawn can move two squares
            if RANK_SQUARES[sqi] == RANK_2 && pos.pieces[sqi + 20] == EMPTY {
                add_quiet_move(
                    pos,
                    GameMove::new(sq, sq + 20, EMPTY, EMPTY, MFLAG_PS),
                    list,
                )
            }
        }

        if is_sq_on_board(sq) && is_black(pos.pieces[sqi + 9]) {
            add_wp_capture_move(pos, sq, sq + 9, pos.pieces[sqi + 9], list);
        }
        if is_sq_on_board(sq) && is_black(pos.pieces[sqi + 11]) {
            add_wp_capture_move(pos, sq, sq + 11, pos.pieces[sqi + 11], list);
        }

        if pos.en_passant != NO_SQ {
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
            if RANK_SQUARES[sqi] == RANK_7 && pos.pieces[sqi - 20] == EMPTY {
                add_quiet_move(
                    pos,
                    GameMove::new(sq, sq - 20, EMPTY, EMPTY, MFLAG_PS),
                    list,
                )
            }
        }

        if is_sq_on_board(sq) && is_white(pos.pieces[sqi - 9]) {
            add_bp_capture_move(pos, sq, sq - 9, pos.pieces[sqi - 9], list);
        }
        if is_sq_on_board(sq) && is_white(pos.pieces[sqi - 11]) {
            add_bp_capture_move(pos, sq, sq - 11, pos.pieces[sqi - 11], list);
        }

        if (pos.en_passant != NO_SQ) {
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
    let mut piece_idx = LOOP_SLIDE_INDEX[side as usize]; // Start at different points in loop array depending on color
    let mut piece = LOOP_SLIDE[piece_idx] as usize;

    while piece != 0 {
        for i in 0..pos.num_pieces[piece as usize] as usize {
            let sq = pos.piece_list[piece][i] as i32;
            for j in 0..NUM_DIR[piece] {
                let dir = PIECE_DIR[piece][j];
                let mut t_sq = sq + dir;

                while is_sq_on_board(t_sq as u8) {
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
                if !is_sq_on_board(t_sq as u8) {
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

#[inline(always)]
fn generate_castle_move(pos: &Board, side: u8, sqs: &[u8], from: u8, to: u8, list: &mut Vec<GameMove>) {
    if !sqs.iter().any(|x| pos.pieces[*x as usize] != EMPTY) && !sqs.iter().any(|x| square_is_attacked(*x, side, &pos.pieces)) {
        add_quiet_move(
            pos,
            GameMove::new(from, to, EMPTY, EMPTY, MFLAG_CA),
            list,
        );
    }
}

pub fn generate_all_moves(pos: &Board, list: &mut Vec<GameMove>) {

    if pos.side == WHITE {
        generate_wp_moves(pos, list); // Pawns have a lot of special rules for movement, best to write specific functions

        if pos.castle_perm & WK_CASTLE != 0 {
            generate_castle_move(&pos, BLACK, &vec!(F1, G1), E1, G1, list);
        }

        if pos.castle_perm & WQ_CASTLE != 0 {
            generate_castle_move(&pos, BLACK, &vec!(D1, C1, B1), E1, C1, list);
        }

        generate_sliding_moves(pos, list, WHITE);
        generate_nonsliding_moves(pos, list, WHITE);
    } else {
        generate_bp_moves(pos, list);

        if pos.castle_perm & BK_CASTLE != 0 {
            generate_castle_move(&pos, WHITE, &vec!(F8, G8), G1, G8, list);
        }

        if pos.castle_perm & BQ_CASTLE != 0 {
            generate_castle_move(&pos, WHITE, &vec!(D8, C8, B8), E8, C8, list);
        }

        generate_sliding_moves(pos, list, BLACK);
        generate_nonsliding_moves(pos, list, BLACK);
    }
}

#[cfg(test)]
mod test {
    use crate::constants::{*};
    use crate::moves::movegen::*;
    use crate::moves::gamemove::GameMove;
    use crate::utils::square_utils::fr2sq;
    use crate::game_board::board::Board;
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
            fr2sq(FILE_G, RANK_7),
            fr2sq(FILE_H, RANK_8),
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
            fr2sq(FILE_B, RANK_4),
            fr2sq(FILE_C, RANK_5),
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
            fr2sq(FILE_G, RANK_2),
            fr2sq(FILE_H, RANK_1),
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
            fr2sq(FILE_D, RANK_5),
            fr2sq(FILE_C, RANK_4),
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

    fn init_pces() -> [u8; 120] {
        let mut pces: [u8; 120] = [0; 120];

        let mut sq64_to_sq120: [u8; 64] = [120; 64];

        let mut sq64: usize = 0;
        for rank in RANK_1..RANK_NONE {
            for file in FILE_A..FILE_NONE {
                let sq: u8 = fr2sq(file, rank);
                sq64_to_sq120[sq64] = sq;
                sq64 += 1;
            }
        }

        for i in 0..120 {
            pces[i] = OFFBOARD;
        }
        for i in 0..64 {
            pces[usize::try_from(sq64_to_sq120[i]).unwrap()] = pieces::EMPTY;
        }

        pces[fr2sq(FILE_E, RANK_3) as usize] = pieces::WK;
        pces[fr2sq(FILE_E, RANK_8) as usize] = pieces::BK;
        pces[fr2sq(FILE_B, RANK_2) as usize] = pieces::WP;
        pces[fr2sq(FILE_C, RANK_1) as usize] = pieces::BN;
        pces[fr2sq(FILE_H, RANK_2) as usize] = pieces::WB;
        pces[fr2sq(FILE_A, RANK_1) as usize] = pieces::BB;
        pces[fr2sq(FILE_E, RANK_6) as usize] = pieces::BR;
        pces[fr2sq(FILE_D, RANK_6) as usize] = pieces::BQ;

        pces
    }

    #[test]
    fn test_with_pawn_attacking() {
        let pces = init_pces();
        assert_eq!(square_is_attacked(fr2sq(FILE_C, RANK_3), pieces::WHITE, &pces),
                   true, "Did not correctly find that pawn was attacking a square");
    }

    #[test]
    fn test_without_pawn_attacking() {
        let pces = init_pces();
        assert_eq!(square_is_attacked(fr2sq(FILE_B, RANK_5), pieces::WHITE, &pces),
                   false, "Incorrectly found that a pawn attacking when the pawn was too far away");
    }

    #[test]
    fn test_with_knight_attacking() {
        let pces = init_pces();
        assert_eq!(square_is_attacked(fr2sq(FILE_B, RANK_3), pieces::BLACK, &pces),
                   true, "Did not correctly find that a knight was attacking a square");
        assert_eq!(square_is_attacked(fr2sq(FILE_D, RANK_3), pieces::BLACK, &pces),
                   true, "Did not correctly find that a knight was attacking a square");
        assert_eq!(square_is_attacked(fr2sq(FILE_E, RANK_2), pieces::BLACK, &pces),
                   true, "Did not correctly find that a knight was attacking a square");
        assert_eq!(square_is_attacked(fr2sq(FILE_A, RANK_2), pieces::BLACK, &pces),
                   true, "Did not correctly find that a knight was attacking a square");
    }

    #[test]
    fn test_without_knight_attacking() {
        let pces = init_pces();
        assert_eq!(square_is_attacked(fr2sq(FILE_C, RANK_3), pieces::BLACK, &pces),
                   false, "Incorrectly found a knight was attacking a square");
        assert_eq!(square_is_attacked(fr2sq(FILE_C, RANK_4), pieces::BLACK, &pces),
                   false, "Incorrectly find that a knight was attacking a square");
        assert_eq!(square_is_attacked(fr2sq(FILE_F, RANK_2), pieces::BLACK, &pces),
                   false, "Incorrectly find that a knight was attacking a square");
        assert_eq!(square_is_attacked(fr2sq(FILE_A, RANK_4), pieces::BLACK, &pces),
                   false, "Incorrectly find that a knight was attacking a square");
    }

    #[test]
    fn test_with_bishop_attacking() {
        let pces = init_pces();
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_G, RANK_3),
                                           pieces::WHITE, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_F, RANK_4),
                                           pieces::WHITE, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_E, RANK_5),
                                           pieces::WHITE, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_D, RANK_6),
                                           pieces::WHITE, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_B, RANK_2),
                                           pieces::BLACK, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_B, RANK_8),
                                           pieces::BLACK, &pces), true,
                   "Did not find queen attacking the diagonal square");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_H, RANK_2),
                                           pieces::BLACK, &pces), true,
                   "Did not find queen attacking the diagonal square");
    }

    #[test]
    fn test_without_bishop_attacking() {
        let mut pces = init_pces();
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_C, RANK_3),
                                           pieces::BLACK, &pces), false,
                   "Incorrectly found the bishop attacking a square past a pawn");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_C, RANK_7),
                                           pieces::WHITE, &pces), false,
                   "Incorrectly found a bishop attacking past a piece");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_B, RANK_8),
                                           pieces::WHITE, &pces), false,
                   "Incorrectly found a bishop attacking past a piece");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_A, RANK_5),
                                           pieces::BLACK, &pces), false,
                   "Incorrectly found a bishop attacking in straight line");
    }

    #[test]
    fn test_with_rook_attacking() {
        let pces = init_pces();
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_A, RANK_6),
                                           pieces::BLACK, &pces), true,
                   "Did not correctly find the rook attacking a square");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_E, RANK_3),
                                           pieces::BLACK, &pces), true,
                   "Did not correctly find the rook attacking a square");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_A, RANK_6),
                                           pieces::BLACK, &pces), true,
                   "Did not correctly find the queen attacking a linear square");
    }

    #[test]
    fn test_without_rook_attacking() {
        let pces = init_pces();

        assert_eq!(sliding_piece_attacking(fr2sq(FILE_G, RANK_8),
                                           pieces::BLACK, &pces), false,
                   "Incorrectly found a square diagonal to rook being attacked");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_C, RANK_4),
                                           pieces::BLACK, &pces), false,
                   "Incorrectly found a square diagonal to rook being attacked");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_E, RANK_2),
                                           pieces::BLACK, &pces), false,
                   "Incorrectly found a square being attacked by rook that's blocked by a piece");
        assert_eq!(sliding_piece_attacking(fr2sq(FILE_H, RANK_6),
                                           pieces::BLACK, &pces), true,
                   "Incorrectly find the queen attacking past a piece");
    }

    #[test]
    fn test_with_king_attacking() {
        let pces = init_pces();
        assert_eq!(square_is_attacked(fr2sq(FILE_E, RANK_4),
                                      pieces::WHITE, &pces), true,
                   "Did not correctly find the king attacking a square");
        assert_eq!(square_is_attacked(fr2sq(FILE_F, RANK_7),
                                      pieces::BLACK, &pces), true,
                   "Did not correctly find the king attacking a square");
    }

    #[test]
    fn test_without_king_attacking() {
        let pces = init_pces();
        assert_eq!(square_is_attacked(fr2sq(FILE_D, RANK_5),
                                      pieces::WHITE, &pces), false,
                   "incorrectly find the rook attacking a square");
        assert_eq!(square_is_attacked(fr2sq(FILE_F, RANK_7),
                                      pieces::WHITE, &pces), false,
                   "incorrectly find the king of the wrong color attacking a square");
    }
}

