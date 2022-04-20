/// Code used for storing the general state of the board
pub const BRD_SQ_NUM: usize = 120;

enum Pieces { EMPTY, WP, WN, WB, WQ, WK, BP, BN, BB, BQ, BK }
enum Files { FileA=0, FileB, FileC, FileD, FileE, FileF, FileG, FileH, FileNone }
pub enum Ranks { Rank1=0, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8, RankNone }

enum Squares {
        A1 = 21, B1, C1, D1, E1, F1, G1, H1,
        A2 = 31, B2, C2, D2, E2, F2, G2, H2,
        A3 = 41, B3, C3, D3, E3, F3, G3, H3,
        A4 = 51, B4, C4, D4, E4, F4, G4, H4,
        A5 = 61, B5, C5, D5, E5, F5, G5, H5,
        A6 = 71, B6, C6, D6, E6, F6, G6, H6,
        A7 = 81, B7, C7, D7, E7, F7, G7, H7,
        A8 = 91, B8, C8, D8, E8, F8, G8, H8, NoSq
}

enum Castling { WKingCastle = 1, WQueenCastle = 2, BKingCastle = 3, BQueenCastle = 4}

pub fn fr2sq(file:u64, rank:u64) -> u64 {
        (21 + file) + (rank * 10)
}

pub struct PastMove {
        moved: u64,
        en_passant: u64,
        castle_perm: u8, //Castle permission
        fifty_move: u64,
        pos_key: u64,

}

pub struct Board {
        pieces: [u8; BRD_SQ_NUM],
        pawns: [u64; 3],
        king_sq: [u8; 2],
        fifty_move: u64,
        side: u8,
        en_passant: u64,

        ply: u64,
        history_ply: u64,

        castle_perm: u8, //Castle permission

        pos_key: u64,

        num_pieces: [u8; 13],
        num_big_pieces: [u8; 3],
        num_major_pieces: [u8; 3],
        num_minor_pieces: [u8; 3],

        history: Vec<PastMove>,

        pub sq120_to_sq64: [u8; 120], // Array to convert 10x12 square numbers to 8x8 square numbers
        pub sq64_to_sq120: [u8; 64], //Array to convert 8x8 square numbers to 10x12 square numbers
}

impl Board {
    pub fn new() -> Board {
            let mut sq120_to_sq64:[u8; 120] = [65;120];
            let mut sq64_to_sq120:[u8; 64] = [120; 64];
            let mut sq64:usize = 0;
            for rank in Ranks::Rank1 as u64 .. Ranks::RankNone as u64 {
                    for file in Files::FileA as u64 .. Files::FileNone as u64 {
                            let sq:u64 = fr2sq(file, rank);
                            sq64_to_sq120[sq64] = sq as u8;
                            sq120_to_sq64[sq as usize] = sq64 as u8;
                            sq64 += 1;
                    }
            }
            Board {
                    pieces: [0; 120],
                    pawns: [0; 3],
                    king_sq: [0; 2],
                    fifty_move: 0,
                    side: 0,
                    en_passant: 0,
                    ply: 0,
                    history_ply: 0,
                    castle_perm: 0,
                    pos_key: 0,
                    num_pieces: [0; 13],
                    num_big_pieces: [0; 3],
                    num_major_pieces: [0; 3],
                    num_minor_pieces: [0; 3],
                    history: vec![],
                    sq120_to_sq64,
                    sq64_to_sq120
            }
    }
}

#[cfg(test)]
mod test {
        use crate::board::fr2sq;

        #[test]
        fn test_fr2sq() {
                let square = fr2sq(3, 5);
                assert_eq!(square,
                           74,
                           "Did not convert file and rank into correct square"
                );
        }

        use crate::board::Board;

        #[test]
        fn test_new_board() {
                let board = Board::new();
                assert_eq!(board.sq120_to_sq64[32], 9, "Did not correctly identify 64 square board numbers");
                assert_eq!(board.sq120_to_sq64[0], 65, "Off board values have incorrect values");
                let mut sum:i32 = 0;
                for value in board.sq120_to_sq64 {
                        sum += value as i32;
                }
                assert_eq!(sum, 5656, "Sum of sq120_to_s64 contents not correct");

                assert_eq!(board.sq64_to_sq120[21], 46, "Did not correctly identify 120 square board numbers");
                sum = 0;
                for value in board.sq64_to_sq120 {
                        sum += value as i32;
                }
                assert_eq!(sum, 3808, "Sum of sq64_to_s120 contents not correct");
        }
}

