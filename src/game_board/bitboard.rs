/// Module for bitboards which use a 64 bit unsigned integer where each bit represents the
/// presence of a piece for one particular square.

use crate::utils::bit_operations::bitscan_forward;
use crate::game_board::board::Board;
use crate::constants::{squares};
use crate::utils::square_utils::fr2sq;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BitBoard {
    pub board: u64,
    set_mask: [u64; 64],
    clear_mask: [u64; 64],
}

impl BitBoard {
    pub fn new(board: u64) -> BitBoard {

        //Set and clear masks are used to quickly change individual bits in bitboard
        // with bitwise operations
        let mut set_mask = [0; 64];
        let mut clear_mask = [0; 64];

        for i in 0..64 {
            set_mask[i] |= 1 << i as u64;
            clear_mask[i] = !set_mask[i];
        }

        BitBoard {
            board,
            set_mask,
            clear_mask,
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) { self.board = 0; }

    #[inline(always)]
    pub fn set_bit(&mut self, sq: u8) {
        self.board |= self.set_mask[sq as usize]
    }

    #[inline(always)]
    pub fn clear_bit(&mut self, sq: u8) {
        self.board &= self.clear_mask[sq as usize]
    }

    #[inline(always)]
    pub fn move_bit(&mut self, from: u8, to:u8) {
        self.board ^= (self.set_mask[to as usize] | self.set_mask[from as usize])
    }

    #[inline(always)]
    /// Removes least significant bit and returns its index
    pub fn pop_bit(&mut self) -> u8 {
        let index: u8 = bitscan_forward(self.board);
        self.board ^= 1 << index;
        index
    }

    #[inline(always)]
    /// Counts the number of 1 bits in the bitboard
    pub fn count_bits(&mut self) -> u8 {
        self.board.count_ones() as u8
    }

    #[inline(always)]
    /// Checks if a piece is present at the given 64 square index
    pub fn piece_is_present(self, sq64: u8) -> bool {
        (1 << sq64) & self.board > 0
    }
}

/// Prints the bitboard out on 8x8 grid with x marking the presence of a piece
/// and - marking an empty square
impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let board = Board::new();
        let mut output = String::from("");
        for rank in squares::RANK_1..squares::RANK_NONE {
            for file in squares::FILE_A..squares::FILE_NONE {
                let sq: u8 = board.sq64(fr2sq(file, rank));
                output.push_str(if self.piece_is_present(sq) { "x " } else { "- " });
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}


#[cfg(test)]
mod test {
    use crate::game_board::bitboard::BitBoard;

    #[test]
    fn test_reset() {
        let initial_bits: u64 = 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000;
        let mut board = BitBoard::new(initial_bits);
        board.reset();
        assert_eq!(board.board, 0, "Did not correctly reset board");
    }

    #[test]
    fn test_set_bit() {
        let initial_bits: u64 = 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000;
        let expected_board: u64 = 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000100;
        let mut board: BitBoard = BitBoard::new(initial_bits);
        board.set_bit(2);
        assert_eq!(board.clone().board,
                   expected_board,
                   "Does not correctly set a bit"
        );
        board.set_bit(2);
        assert_eq!(board.board,
                   expected_board,
                   "Does not correctly leave a set bit set"
        );
    }

    #[test]
    fn test_clear_bit() {
        let initial_bits: u64 = 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000100;
        let expected_board: u64 = 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000;
        let mut board: BitBoard = BitBoard::new(initial_bits);
        board.clear_bit(2);
        assert_eq!(board.clone().board,
                   expected_board,
                   "Does not correctly clear a bit"
        );
        board.clear_bit(2);
        assert_eq!(board.clone().board,
                   expected_board,
                   "Does not correctly leave a clear bit cleared"
        );
    }

    #[test]
    fn test_move_bit() {
        let initial_bits: u64 = 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000100;
        let expected_board: u64 = 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00001000;
        let mut board: BitBoard = BitBoard::new(initial_bits);
        board.move_bit(2, 3);
        assert_eq!(board.board,
                   expected_board,
                   "Does not correctly clear a bit"
        );
    }

    #[test]
    fn test_piece_is_present() {
        let initial_bits: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
        let board: BitBoard = BitBoard::new(initial_bits);
        assert_eq!(board.clone().piece_is_present(0),
                   true,
                   "Does not correctly find piece in occupied square"
        );
        assert_eq!(board.piece_is_present(2),
                   false,
                   "Incorrectly finds piece in empty square."
        );
    }

    #[test]
    fn test_count_bits() {
        let initial_bits: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
        let mut board: BitBoard = BitBoard::new(initial_bits);
        assert_eq!(board.count_bits(),
                   8,
                   "Did not count correct number of bits"
        );
    }

    #[test]
    fn test_pop_bit() {
        let initial_bits: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00100000;
        let mut board: BitBoard = BitBoard::new(initial_bits);
        let index = board.pop_bit();
        assert_eq!(board.count_bits(),
                   7,
                   "Did not remove correct number of bits"
        );
        assert_eq!(board.board,
                   0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000000,
                   "Did not remove correct bit"
        );
        assert_eq!(index, 5, "Did not return correct index");
    }
}