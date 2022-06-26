use crate::constants::pieces::{KING_NUMBER, KNIGHT_NUMBER, PAWN_NUMBER};

#[inline(always)]
pub fn piece_is_pawn(piece:u8) -> bool { PAWN_NUMBER[piece as usize] }

#[inline(always)]
pub fn piece_is_knight(piece:u8) -> bool { KNIGHT_NUMBER[piece as usize] }

#[inline(always)]
pub fn piece_is_king(piece:u8) -> bool { KING_NUMBER[piece as usize] }

#[cfg(test)]
mod test {
    use crate::constants::pieces::{BK, BN, BP, WK, WN, WP, WR};
    use crate::utils::hashkeys::BoardHasher;
    use crate::utils::piece_utils::{piece_is_king, piece_is_knight, piece_is_pawn};

    #[test]
    fn test_piece_is_pawn() {
        assert_eq!(piece_is_pawn(WP), true, "Did not find white pawn as a pawn");
        assert_eq!(piece_is_pawn(BP), true, "Did not find black pawn as a pawn");
        assert_eq!(piece_is_pawn(WR), false, "Found a white rook as a pawn");
    }

    #[test]
    fn test_piece_is_knight() {
        assert_eq!(piece_is_knight(WN), true, "Did not find white knight as a knight");
        assert_eq!(piece_is_knight(BN), true, "Did not find black knight as a knight");
        assert_eq!(piece_is_knight(WR), false, "Found a white rook as a knight");
    }

    #[test]
    fn test_piece_is_king() {
        assert_eq!(piece_is_king(WK), true, "Did not find white king as a king");
        assert_eq!(piece_is_king(BK), true, "Did not find black king as a king");
        assert_eq!(piece_is_king(WR), false, "Found a white rook as a king");
    }


}