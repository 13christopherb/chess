use crate::Board;
use crate::constants::{pieces, sqs};
use crate::constants::sqs::OFFBOARD;

const KN_DIR:[i8; 8] = [ -8, -19, -21, -12, 8, 19, 21, 12]; // Knight attack direction
const RK_DIR:[i8; 4] = [ -1, -10, 1, 10]; // Rook attack direction
const BI_DIR:[i8; 4] = [ -9, -11, 11, 9]; // Bishop attack direction
const KI_DIR:[i8; 8] = [-1, -10, 1, 10, -9, -11, 11, 9]; // King attack direction

#[inline(always)]
fn is_pawn_attacking(sq:u8, side:u8, pces:&[u8; 120]) -> bool {
    let is_attacked;
    if side == pieces::WHITE {
        is_attacked = pces[(sq - 11) as usize] == pieces::WP || pces[(sq - 9) as usize] == pieces::WP;
    } else {
        is_attacked = pces[(sq+  11) as usize] == pieces::BP || pces[(sq + 9) as usize] == pieces::BP;
    }
    is_attacked
}

#[inline(always)]
fn is_knight_attacking(sq:u8, side:u8, pces:&[u8; 120]) -> bool {
    for dir in KN_DIR {
        let pce = pces[(sq as i8 + dir) as usize];
        if pce != sqs::OFFBOARD && pieces::is_knight(pce) && pieces::is_same_color(pce, side) {
            return true
        }
    }
    false
}

#[inline(always)]
fn is_rook_or_queen_attacking(sq:u8, side:u8, pces:&[u8;120]) -> bool {
    for dir in RK_DIR {
        let mut dir_sq = sq as i8 + dir;
        let mut pce = pces[dir_sq as usize];
        while pce != sqs::OFFBOARD {
            if pce != pieces::EMPTY {
                if pieces::is_rook_or_queen(pce) && pieces::is_same_color(pce, side) {
                    return true;
                }
                break;
            }
            dir_sq += dir;
            pce = pces[dir_sq as usize];
        }
    }
    false
}

fn is_bishop_or_queen_attacking(sq:u8, side:u8, pces:&[u8; 120]) -> bool {
    for dir in BI_DIR {
        let mut dir_sq = sq as i8 + dir;
        let mut pce = pces[dir_sq as usize];
        while pce != sqs::OFFBOARD {
            if pce != pieces::EMPTY {
                if pieces::is_bishop_or_queen(pce) && pieces::is_same_color(pce, side) {
                    return true;
                }
                break;
            }
            dir_sq += dir;
            pce = pces[dir_sq as usize];
        }
    }
    false
}

fn is_king_attacking(sq:u8, side:u8, pces:&[u8;120]) -> bool {
    for dir in KI_DIR {
        let pce = pces[(sq as i8 + dir) as usize];
        if pce == OFFBOARD { break; }
        if pieces::is_king(pce) && pieces::is_same_color(pce, side) {
            return true;
        }
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
pub fn is_square_attacked(sq:u8, side:u8, pces: &[u8; 120]) -> bool  {
    is_pawn_attacking(sq, side, pces) || is_knight_attacking(sq, side, pces) ||
        is_rook_or_queen_attacking(sq, side, pces) || is_bishop_or_queen_attacking(sq, side, pces) ||
        is_king_attacking(sq, side, pces)
}

#[cfg(test)]
mod test {
    use crate::constants::{files, pieces, ranks};
    use crate::move_gen::attack::*;
    use crate::utils::square_utils::fr2sq;

    fn init_pces() -> [u8; 120] {
        let mut pces:[u8; 120] = [0; 120];

        let mut sq64_to_sq120: [u8; 64] = [120; 64];

        let mut sq64: usize = 0;
        for rank in ranks::RANK_1..ranks::RANK_NONE {
            for file in files::FILE_A..files::FILE_NONE {
                let sq: u8 = fr2sq(file, rank);
                sq64_to_sq120[sq64] = sq;
                sq64 += 1;
            }
        }

        for i in 0..120 {
            pces[i] = sqs::OFFBOARD;
        }
        for i in 0..64 {
            pces[usize::try_from(sq64_to_sq120[i]).unwrap()] = pieces::EMPTY;
        }
        pces[fr2sq(files::FILE_E, ranks::RANK_3) as usize] = pieces::WK;
        pces[fr2sq(files::FILE_E, ranks::RANK_8) as usize] = pieces::BK;
        pces[fr2sq(files::FILE_B, ranks::RANK_2) as usize] = pieces::WP;
        pces[fr2sq(files::FILE_C, ranks::RANK_1) as usize] = pieces::BN;
        pces[fr2sq(files::FILE_H, ranks::RANK_2) as usize] = pieces::WB;
        pces[fr2sq(files::FILE_A, ranks::RANK_1) as usize] = pieces::BB;
        pces[fr2sq(files::FILE_E, ranks::RANK_6) as usize] = pieces::BR;
        pces[fr2sq(files::FILE_D, ranks::RANK_6) as usize] = pieces::BQ;


        pces
    }

    #[test]
    fn test_with_pawn_attacking() {
        let pces = init_pces();
        assert_eq!(is_pawn_attacking(fr2sq(files::FILE_C, ranks::RANK_3), pieces::WHITE, &pces),
                   true, "Did not correctly find that pawn was attacking a square");
    }
    #[test]
    fn test_without_pawn_attacking() {
        let pces = init_pces();
        assert_eq!(is_pawn_attacking(fr2sq(files::FILE_D, ranks::RANK_3), pieces::WHITE, &pces),
                   false, "Incorrectly found that a pawn attacking when the pawn was too far away");
    }
    #[test]
    fn test_with_knight_attacking() {
        let pces = init_pces();
        assert_eq!(is_knight_attacking(fr2sq(files::FILE_B, ranks::RANK_3), pieces::BLACK, &pces),
                   true, "Did not correctly find that a knight was attacking a square");
        assert_eq!(is_knight_attacking(fr2sq(files::FILE_D, ranks::RANK_3), pieces::BLACK, &pces),
                   true, "Did not correctly find that a knight was attacking a square");
        assert_eq!(is_knight_attacking(fr2sq(files::FILE_E, ranks::RANK_2), pieces::BLACK, &pces),
                   true, "Did not correctly find that a knight was attacking a square");
        assert_eq!(is_knight_attacking(fr2sq(files::FILE_A, ranks::RANK_2), pieces::BLACK, &pces),
                   true, "Did not correctly find that a knight was attacking a square");
    }
    #[test]
    fn test_without_knight_attacking() {
        let pces = init_pces();
        assert_eq!(is_knight_attacking(fr2sq(files::FILE_C, ranks::RANK_3), pieces::BLACK, &pces),
                   false, "Incorrectly found a knight was attacking a square");
        assert_eq!(is_knight_attacking(fr2sq(files::FILE_E, ranks::RANK_3), pieces::BLACK, &pces),
                   false, "Incorrectly find that a knight was attacking a square");
        assert_eq!(is_knight_attacking(fr2sq(files::FILE_F, ranks::RANK_2), pieces::BLACK, &pces),
                   false, "Incorrectly find that a knight was attacking a square");
        assert_eq!(is_knight_attacking(fr2sq(files::FILE_B, ranks::RANK_2), pieces::BLACK, &pces),
                   false, "Incorrectly find that a knight was attacking a square");
    }
    #[test]
    fn test_with_bihsop_attacking() {
        let pces = init_pces();
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_G, ranks::RANK_3),
                                                pieces::WHITE, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_F, ranks::RANK_4),
                                                pieces::WHITE, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_E, ranks::RANK_5),
                                                pieces::WHITE, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_D, ranks::RANK_6),
                                                pieces::WHITE, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_B, ranks::RANK_2),
                                                pieces::BLACK, &pces), true,
                   "Incorrectly did not find a bishop was attacking a square");
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_B, ranks::RANK_8),
                                                pieces::BLACK, &pces), true,
                   "Did not find queen attacking the diagonal square");
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_H, ranks::RANK_2),
                                                pieces::BLACK, &pces), true,
                   "Did not find queen attacking the diagonal square");
    }
    #[test]
    fn test_without_bihsop_attacking() {
        let pces = init_pces();
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_C, ranks::RANK_3),
                                                pieces::BLACK, &pces), false,
                   "Incorrectly found the bishop attacking a square past a pawn");
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_C, ranks::RANK_7),
                                                pieces::WHITE, &pces), false,
                   "Incorrectly found a bishop attacking past a piece");
        assert_eq!(is_bishop_or_queen_attacking(fr2sq(files::FILE_B, ranks::RANK_8),
                                                pieces::WHITE, &pces), false,
                   "Incorrectly found a bishop attacking past a piece");
    }
    #[test]
    fn test_with_rook_attacking() {
        let pces = init_pces();
        assert_eq!(is_rook_or_queen_attacking(fr2sq(files::FILE_A, ranks::RANK_6),
                                                pieces::BLACK, &pces), true,
                   "Did not correctly find the rook attacking a square");
        assert_eq!(is_rook_or_queen_attacking(fr2sq(files::FILE_E, ranks::RANK_3),
                                                pieces::BLACK, &pces), true,
                   "Did not correctly find the rook attacking a square");
        assert_eq!(is_rook_or_queen_attacking(fr2sq(files::FILE_A, ranks::RANK_6),
                                              pieces::BLACK, &pces), true,
                   "Did not correctly find the queen attacking a linear square");
    }
    #[test]
    fn test_without_rook_attacking() {
        let pces = init_pces();
        assert_eq!(is_rook_or_queen_attacking(fr2sq(files::FILE_C, ranks::RANK_7),
                                                pieces::BLACK, &pces), false,
                   "Incorrectly found a square diagonal to rook being attacked");
        assert_eq!(is_rook_or_queen_attacking(fr2sq(files::FILE_E, ranks::RANK_2),
                                                pieces::BLACK, &pces), false,
                   "Incorrectly found a square being attacked by rook that's blocked by a piece");
        assert_eq!(is_rook_or_queen_attacking(fr2sq(files::FILE_H, ranks::RANK_6),
                                              pieces::BLACK, &pces), true,
                   "Incorrectly find the queen attacking past a piece");
    }
    #[test]
    fn test_with_king_attacking() {
        let pces = init_pces();
        assert_eq!(is_king_attacking(fr2sq(files::FILE_E, ranks::RANK_4),
                                              pieces::WHITE, &pces), true,
                   "Did not correctly find the king attacking a square");
        assert_eq!(is_king_attacking(fr2sq(files::FILE_F, ranks::RANK_7),
                                              pieces::BLACK, &pces), true,
                   "Did not correctly find the king attacking a square");
    }
    #[test]
    fn test_without_king_attacking() {
        let pces = init_pces();
        assert_eq!(is_king_attacking(fr2sq(files::FILE_E, ranks::RANK_5),
                                     pieces::WHITE, &pces), false,
                   "incorrectly find the rook attacking a square");
        assert_eq!(is_king_attacking(fr2sq(files::FILE_F, ranks::RANK_7),
                                     pieces::WHITE, &pces), false,
                   "incorrectly find the king of the wrong color attacking a square");
    }
}