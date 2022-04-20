use crate::Board;
use crate::board::Ranks;
use crate::board::Files;
use crate::board::fr2sq;


/// Bits for each rank
pub const RANK_1: u64 = 0x0000_0000_0000_00FF;
pub const RANK_2: u64 = 0x0000_0000_0000_FF00;
pub const RANK_3: u64 = 0x0000_0000_00FF_0000;
pub const RANK_4: u64 = 0x0000_0000_FF00_0000;
pub const RANK_5: u64 = 0x0000_00FF_0000_0000;
pub const RANK_6: u64 = 0x0000_FF00_0000_0000;
pub const RANK_7: u64 = 0x00FF_0000_0000_0000;
pub const RANK_8: u64 = 0xFF00_0000_0000_0000;

/// Bits for each file
pub const FILE_A:u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
pub const FILE_B:u64 = 0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;
pub const FILE_C:u64 = 0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100;
pub const FILE_D:u64 = 0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000;
pub const FILE_E:u64 = 0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000;
pub const FILE_F:u64 = 0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000;
pub const FILE_G:u64 = 0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000;

#[derive(Debug, Copy, Clone)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Counts the number of 1 bits in the bitboard
    pub fn count_bits(self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Checks if a piece is present at the given 64 square index
    pub fn piece_is_present(self, sq64:u64) -> bool {
        (1 << sq64) & self.0 > 0
    }
}

/// Prints the bitboard out on 8x8 grid with x marking the presence of a piece
/// and - marking an empty square
impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let board = Board::new();
        let mut output = String::from("");
        for rank in Ranks::Rank1 as u64 .. Ranks::RankNone as u64 {
            for file in Files::FileA as u64 .. Files::FileNone as u64 {
                let sq:u64 = board.sq120_to_sq64[fr2sq(file, rank) as usize];
                output.push_str(if self.piece_is_present(sq) {"x "} else {"- "} );
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}


#[cfg(test)]
mod test {
    use crate::bitboard::BitBoard;

    #[test]
    fn piece_is_present() {
        let initial_bits:u64 =  0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
        let board:BitBoard = BitBoard(initial_bits);
        assert_eq!(board.clone().piece_is_present(0),
                   true,
                   "Does not correctly find piece in occupied square"
        );
        assert_eq!(board.piece_is_present(2),
                   false,
                   "Incorrectly finds piece in empty square"
        );
    }

    #[test]
    fn test_count_bits() {
        let initial_bits:u64 =  0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
        let board:BitBoard = BitBoard(initial_bits);
        assert_eq!(board.count_bits(),
                   8,
                   "New matrix did not contain correct data"
        );
    }
}