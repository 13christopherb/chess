use crate::bitboard::bitboard::BitBoard;
use crate::constants::{files, piece_values, pieces, ranks, squares};
use crate::hashkeys::hash_keys::BoardHasher;

/// Code used for storing the general state of the board

enum Castling { WKingCastle = 1, WQueenCastle = 2, BKingCastle = 3, BQueenCastle = 4 }

pub fn fr2sq(file: u8, rank: u8) -> u8 {
    (21 + file) + (rank * 10)
}

pub struct PastMove {
    moved: u64,
    en_passant: u64,
    castle_perm: u8,
    //Castle permission
    fifty_move: u64,
    pos_key: u64,

}

pub struct Board {
    pieces: [u8; 120],
    pawns: [BitBoard; 3],
    king_sq: [u8; 2],
    fifty_move: u64,
    side: u8,
    en_passant: u8,

    ply: u64,
    history_ply: u64,

    castle_perm: u8, //Castle permission

    hash_key: u64,

    num_pieces: [u8; 13],
    num_big_pieces: [u8; 2],
    num_major_pieces: [u8; 2],
    num_minor_pieces: [u8; 2],
    material: [u32; 2],

    history: Vec<PastMove>,

    pub sq120_to_sq64: [u8; 120],
    // Array to convert 10x12 square numbers to 8x8 square numbers
    pub sq64_to_sq120: [u8; 64], //Array to convert 8x8 square numbers to 10x12 square numbers

    piece_list: [[u8; 18]; 13],

    hasher: BoardHasher,

}

impl Board {
    pub fn new() -> Board {
        let mut sq120_to_sq64: [u8; 120] = [65; 120];
        let mut sq64_to_sq120: [u8; 64] = [120; 64];
        let mut sq64: usize = 0;
        for rank in ranks::RANK_1..ranks::RANK_NONE {
            for file in files::FILE_A..files::FILE_NONE {
                let sq: u8 = fr2sq(file, rank);
                sq64_to_sq120[sq64] = sq;
                sq120_to_sq64[sq as usize] = sq64 as u8;
                sq64 += 1;
            }
        }
        Board {
            pieces: [0; 120],
            pawns: [BitBoard::new(0); 3],
            king_sq: [0; 2],
            fifty_move: 0,
            side: 0,
            en_passant: 0,
            ply: 0,
            history_ply: 0,
            castle_perm: 0,
            hash_key: 0,
            num_pieces: [0; 13],
            num_big_pieces: [0; 2],
            num_major_pieces: [0; 2],
            num_minor_pieces: [0; 2],
            material: [0; 2],
            history: vec![],
            sq120_to_sq64,
            sq64_to_sq120,
            piece_list: [[0; 18]; 13],
            hasher: BoardHasher::new(),
        }
    }

    /// Resets the position to an empty board
    pub fn reset_position(&mut self) {
        for i in 0..120 {
            self.pieces[i] = squares::OFFBOARD;
        }
        for i in 0..64 {
            self.pieces[usize::try_from(self.sq64_to_sq120[i]).unwrap()] = pieces::EMPTY;
        }

        for i in 0..2 {
            self.num_big_pieces[i] = 0;
            self.num_major_pieces[i] = 0;
            self.num_minor_pieces[i] = 0;
            self.pawns[i] = BitBoard::new(0);
        }

        for i in 0..13 {
            self.num_pieces[i] = 0;
        }

        self.king_sq[0] = 0;
        self.king_sq[1] = 0;

        self.side = 2;
        self.en_passant = squares::NO_SQ;
        self.fifty_move = 0;

        self.ply = 0;
        self.history_ply = 0;

        self.castle_perm = 0;

        self.hash_key = 0;
    }

    pub fn update_material_list(&mut self) {
        let mut sq = 0;
        for i in 0..120 {
            sq = i as u8;
            let mut piece = self.pieces[i];
            let mut color: usize;
            if piece != squares::OFFBOARD && piece != pieces::EMPTY {
                color = piece_values::PIECE_COLOR[piece as usize] as usize;

                if piece_values::BIG_PIECE[piece as usize] { self.num_big_pieces[color] += 1;}
                if piece_values::MINOR_PIECE[piece as usize] { self.num_minor_pieces[color] += 1;}
                if piece_values::MAJOR_PIECE[piece as usize] { self.num_major_pieces[color] += 1;}

                self.material[color] += piece_values::VALUE[piece as usize];

                self.piece_list[piece as usize][self.num_pieces[piece as usize] as usize] = sq;
                self.num_pieces[piece as usize] += 1;

                if piece == pieces::WK {self.king_sq[color] = sq;}
                if piece == pieces::BK {self.king_sq[color] = sq;}

            }
        }
    }

    /// Parses a string containing a Forsyth–Edwards Notation position and sets
    /// the board's state to match the string.
    /// # Panic
    /// Should panic if the string is not a valid FEN
    pub unsafe fn parse_fen(&mut self, fen: &str) {
        let mut rank: i32 = ranks::RANK_8 as i32;
        let mut file: i32 = files::FILE_A as i32;

        let mut piece = 0;
        let mut count = 0;
        let mut sq64 = 0;
        let mut sq120 = 0;

        self.reset_position();

        let mut c = fen.as_ptr();

        while rank >= 0 && *c != 0 {
            count = 1;
            match *c as char {
                'p' => piece = pieces::BP,
                'r' => piece = pieces::BR,
                'n' => piece = pieces::BN,
                'b' => piece = pieces::BB,
                'k' => piece = pieces::BK,
                'q' => piece = pieces::BQ,
                'P' => piece = pieces::WP,
                'R' => piece = pieces::WR,
                'N' => piece = pieces::WN,
                'B' => piece = pieces::WB,
                'K' => piece = pieces::WK,
                'Q' => piece = pieces::WQ,
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    piece = pieces::EMPTY;
                    count = (*c as char).to_digit(10).unwrap();
                },
                '/' | ' ' => {
                    rank -= 1;
                    file = files::FILE_A as i32;
                    c = c.add(1);
                    continue;
                },
                _ => panic!("Inorrectly formatted string"),
            }

            // Set the square for the piece that was found or skip forward to the position
            // of the next piece
            for _ in 0..count {
                sq64 = rank * 8 + file;
                sq120 = self.sq64_to_sq120[sq64 as usize];
                if piece != pieces::EMPTY as u8 {
                    self.pieces[sq120 as usize] = piece;
                }
                file += 1;
            }
            c = c.add(1);
        } // end of while

        assert!(*c as char == 'w' || *c as char == 'b');

        self.side = if *c as char == 'w' { 0 } else { 1 };
        c = c.add(2);

        // Castle permission
        for _ in 0..4 {
            if *c as char == ' ' { break; }

            match *c as char {
                'K' => self.castle_perm |= Castling::WKingCastle as u8,
                'Q' => self.castle_perm |= Castling::WQueenCastle as u8,
                'k' => self.castle_perm |= Castling::BKingCastle as u8,
                'q' => self.castle_perm |= Castling::BQueenCastle as u8,
                _ => break,
            }
            c = c.add(1);
        }
        c = c.add(1);

        // En passant
        if *c as char != '-' {
            match *c as char {
                'a' => file = 0,
                'b' => file = 1,
                'c' => file = 2,
                'd' => file = 3,
                'e' => file = 4,
                'f' => file = 5,
                'g' => file = 6,
                'h' => file = 7,
                _ => panic!("Incorrectly formatted string")
            }
            rank = (*c.add(1) as char).to_digit(10).unwrap() as i32;

            self.en_passant = fr2sq(file as u8, rank as u8);
        }
        self.hash_key = self.hasher.generate_key(self.pieces, self.side, self.en_passant, self.castle_perm);
    }
}

/// Prints the board
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const PIECE_CHARS:[&str; 13] = [" . "," P "," N "," B "," R "," Q "," K ",
            " p "," n "," b "," r "," q "," k "];
        const SIDE_CHARS:[char; 3] = ['w','b','-'];
        const RANK_CHARS:[char; 8] = ['1','2','3','4','5','6','7','8'];
        const FILE_CHARS:[char; 8] = ['a','b','c','d','e','f','g','h'];
        let mut output = String::from("");
        for rank in (ranks::RANK_1..=ranks::RANK_8).rev() {
            for file in files::FILE_A..files::FILE_H + 1 {
                let sq = fr2sq(file, rank);
                let piece = self.pieces[sq as usize];
                output.push_str(PIECE_CHARS[piece as usize]);
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use crate::board::{fr2sq, pieces, squares};

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
        let mut sum: i32 = 0;
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

    #[test]
    fn test_parse_fen() {
        let start: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let white_pawn_bb: u64 = 0x0000_0000_0000_FF00;

        let mut board = Board::new();
        unsafe { board.parse_fen(start) };
        assert_eq!(board.pieces[23], pieces::WB as u8, "Did not correctly place white bishop on F1");
        assert_eq!(board.pieces[87], pieces::BP as u8, "Did not correctly place black pawn on C7");
        assert_eq!(board.pieces[0], squares::OFFBOARD, "Did not preserve offboard values");
        assert_eq!(board.side, 0, "Did not correctly set it as white's move");
        assert_eq!(board.castle_perm, 7, "Did not correctly set castling permission");
        assert_eq!(board.en_passant, squares::NO_SQ, "Did not correctly set en passant square");
        //assert_eq!(board.pawns[0].board, white_pawn_bb, "Did not correctly set position of white pawn bitboard");
    }

    #[test]
    fn test_update_material_list() {
        let start: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut board = Board::new();
        unsafe { board.parse_fen(start) };
        board.update_material_list();
        //King is counted as a major piece
        assert_eq!(board.num_major_pieces[0], 4, "Did not update with correct number of white major pieces");
        assert_eq!(board.num_major_pieces[1], 4, "Did not update with correct number of black major pieces");
        assert_eq!(board.num_minor_pieces[0], 4, "Did not update with correct number of white minor pieces");
        assert_eq!(board.num_minor_pieces[1], 4, "Did not update with correct number of black minor pieces");
        assert_eq!(board.num_big_pieces[0], 8, "Did not update with correct number of white big pieces");
        assert_eq!(board.num_big_pieces[1], 8, "Did not update with correct number of black big pieces");
        assert_eq!(board.material[0], 54200, "Did not correctly set material value for white");
        assert_eq!(board.material[1], 54200, "Did not correctly set material value for black");
        assert_eq!(board.piece_list[1][4], 35, "Did not correctly set square for white pawn");
    }
}

