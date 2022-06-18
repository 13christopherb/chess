use crate::game_board::bitboard::BitBoard;
use crate::constants::{castling, files, pieces, ranks, sqs};
use crate::utils::hashkeys::BoardHasher;
use crate::utils::square_utils::fr2sq;

/// Code used for storing the general state of the board

#[derive(Debug, Copy, Clone)]
pub struct PastMove {
    moved: u64,
    en_passant: u64,
    castle_perm: u8,
    //Castle permission
    fifty_move: u64,
    pos_key: u64,

}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [u8; 120],
    pub bitboards: [BitBoard; 3],
    pub king_sq: [u8; 2],
    fifty_move: u64,
    pub side: u8,
    pub en_passant: u8,

    ply: u64,
    history_ply: u64,

    pub castle_perm: u8, //Castle permission

    hash_key: u64,

    pub num_pieces: [u8; 13],
    pub num_big_pieces: [u8; 2],
    pub num_major_pieces: [u8; 2],
    pub num_minor_pieces: [u8; 2],
    pub material: [u32; 2],

    history: Vec<PastMove>,

    pub sq120_to_sq64: [u8; 120],
    // Array to convert 10x12 square numbers to 8x8 square numbers
    pub sq64_to_sq120: [u8; 64], //Array to convert 8x8 square numbers to 10x12 square numbers

    pub piece_list: [[u8; 18]; 13],

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
            bitboards: [BitBoard::new(0); 3],
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
}
impl Board {

    /// Resets the position to an empty board
    pub fn reset_position(&mut self) {
        for i in 0..120 {
            self.pieces[i] = sqs::OFFBOARD;
        }
        for i in 0..64 {
            self.pieces[usize::try_from(self.sq64_to_sq120[i]).unwrap()] = pieces::EMPTY;
        }

        for i in 0..2 {
            self.num_big_pieces[i] = 0;
            self.num_major_pieces[i] = 0;
            self.num_minor_pieces[i] = 0;
            self.material[i] = 0;
            self.bitboards[i] = BitBoard::new(0);
        }

        for i in 0..3 {
            self.bitboards[i].reset();
        }

        for i in 0..13 {
            self.num_pieces[i] = 0;
        }

        self.king_sq[0] = 0;
        self.king_sq[1] = 0;

        self.side = 2;
        self.en_passant = sqs::NO_SQ;
        self.fifty_move = 0;

        self.ply = 0;
        self.history_ply = 0;

        self.castle_perm = 0;

        self.hash_key = 0;
    }

    /// Updates the rest of the board's state with regards to pieces to match the current piece list
    pub fn update_material_list(&mut self) {
        let mut sq: u8;
        for i in 0..120 {
            sq = i as u8;
            let piece = self.pieces[i];
            let color: usize;
            if piece != sqs::OFFBOARD && piece != pieces::EMPTY {
                color = pieces::PIECE_COLOR[piece as usize] as usize;

                if pieces::BIG_PIECE[piece as usize] { self.num_big_pieces[color] += 1; }
                if pieces::MINOR_PIECE[piece as usize] { self.num_minor_pieces[color] += 1; }
                if pieces::MAJOR_PIECE[piece as usize] { self.num_major_pieces[color] += 1; }

                self.material[color] += pieces::VALUE[piece as usize];

                self.piece_list[piece as usize][self.num_pieces[piece as usize] as usize] = sq;
                self.num_pieces[piece as usize] += 1;

                if piece == pieces::WK || piece == pieces::BK { self.king_sq[color] = sq; }

                if piece == pieces::WP || piece == pieces::BP {
                    self.bitboards[color].set_bit(self.sq120_to_sq64[sq as usize]);
                    self.bitboards[pieces::BOTH as usize].set_bit(self.sq120_to_sq64[sq as usize]);
                }
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

        let mut piece: u8;
        let mut count: u32;
        let mut sq64: i32;
        let mut sq120: u8;

        self.reset_position();

        let mut c = fen.as_ptr();
        let mut i: usize = 0;
        let length = fen.chars().count();

        // Use i to keep track of number of loops in case string isn't formatted right
        while rank >= 0 && i < length {
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
            i += 1;
        } // end of while

        assert!(*c as char == 'w' || *c as char == 'b');

        self.side = if *c as char == 'w' { 0 } else { 1 };
        c = c.add(2);

        // Castle permission
        for _ in 0..4 {
            if *c as char == ' ' { break; }

            match *c as char {
                'K' => self.castle_perm |= castling::WK_CASTLE,
                'Q' => self.castle_perm |= castling::WQ_CASTLE,
                'k' => self.castle_perm |= castling::BK_CASTLE,
                'q' => self.castle_perm |= castling::BQ_CASTLE,
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
            let rank_char: char = *c.add(1) as char;
            rank = rank_char.to_digit(10).unwrap() as i32 - 1; // Rank in string 1 index, so subtract 1
            self.en_passant = fr2sq(file as u8, rank as u8);
        }

        //self.ply =

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

/// Giant function to check if there is anything obviously wrong with the board's state
/// TODO: need to add an assert for a valid hash key
/// # Panic
/// Will panic if any of the board's state seems to be incorrect
pub fn check_board(board:&Board) -> bool {
    let mut num_pieces = [0; 13];
    let mut big_pieces = [0; 2];
    let mut major_pieces = [0; 2];
    let mut minor_pieces = [0; 2];
    let mut material = [0; 2];

    let mut pawns = [BitBoard::new(0); 3];

    pawns[pieces::WHITE_S] = board.bitboards[pieces::WHITE_S];
    pawns[pieces::BLACK_S] = board.bitboards[pieces::BLACK_S];
    pawns[pieces::BOTH_S] = board.bitboards[pieces::BOTH_S];

    // check piece list

    for piece in pieces::WP..=pieces::BK {
        for piece_num in 0..num_pieces[piece as usize] {
            let sq120 = board.piece_list[piece as usize][piece_num] as usize;
            assert_eq!(board.pieces[sq120], piece);
        }
    }

    // check piece counts

    for sq64 in 0..64 {
        let sq120 = board.sq64_to_sq120[sq64] as usize;
        let piece = board.pieces[sq120] as usize;
        if piece as u8 != pieces::EMPTY {
            num_pieces[piece] += 1;
            let color = pieces::PIECE_COLOR[piece] as usize;
            if pieces::BIG_PIECE[piece] { big_pieces[color] += 1; }
            if pieces::MAJOR_PIECE[piece] { major_pieces[color] += 1; }
            if pieces::MINOR_PIECE[piece] { minor_pieces[color] += 1; }

            material[color] += pieces::VALUE[piece];
        }
    }
    for piece in pieces::WP as usize..=pieces::BK as usize {
        assert_eq!(num_pieces[piece] as u8, board.num_pieces[piece]);
    }
    // check bitboards

    let mut pcount = pawns[pieces::WHITE_S].count_bits();
    assert_eq!(pcount, board.num_pieces[pieces::WP as usize]);
    pcount = pawns[pieces::BLACK_S].count_bits();
    assert_eq!(pcount, board.num_pieces[pieces::BP as usize]);
    pcount = pawns[pieces::BOTH_S].count_bits();
    assert_eq!(pcount, board.num_pieces[pieces::BP as usize] + board.num_pieces[pieces::WP as usize]);

    // check bitboards squares

    while pawns[pieces::WHITE_S].board > 0 {
        let sq64 = pawns[pieces::WHITE_S].pop_bit();
        assert_eq!(board.pieces[board.sq64_to_sq120[sq64 as usize] as usize], pieces::WP);
    }

    while pawns[pieces::BLACK_S].board > 0 {
        let sq64 = pawns[pieces::BLACK_S].pop_bit();
        assert_eq!(board.pieces[board.sq64_to_sq120[sq64 as usize] as usize], pieces::BP);
    }

    while pawns[pieces::BOTH_S].board > 0 {
        let sq64 = pawns[pieces::BOTH_S].pop_bit();
        assert!(board.pieces[board.sq64_to_sq120[sq64 as usize] as usize] == pieces::BP ||
            board.pieces[board.sq64_to_sq120[sq64 as usize] as usize] == pieces::WP);
    }

    assert!(material[pieces::WHITE_S] == board.material[pieces::WHITE_S] &&
        material[pieces::BLACK_S] == board.material[pieces::BLACK_S]);
    assert!(minor_pieces[pieces::WHITE_S] == board.num_minor_pieces[pieces::WHITE_S] &&
        minor_pieces[pieces::BLACK_S] == board.num_minor_pieces[pieces::BLACK_S]);
    assert!(major_pieces[pieces::WHITE_S] == board.num_major_pieces[pieces::WHITE_S] &&
        major_pieces[pieces::BLACK_S] == board.num_major_pieces[pieces::BLACK_S]);
    assert!(big_pieces[pieces::WHITE_S] == board.num_big_pieces[pieces::WHITE_S] &&
        big_pieces[pieces::BLACK_S] == board.num_big_pieces[pieces::BLACK_S]);

    assert!(board.side == pieces::WHITE || board.side == pieces::BLACK);

    // assert!(board.en_passant == squares::NO_SQ ||
    //     ( board.ranks_squares[board.en_passant as usize] == ranks::RANK_6 &&
    //         board.side == pieces::WHITE) ||
    //     (board.ranks_squares[board.en_passant as usize] == ranks::RANK_3 &&
    //         board.side == pieces::BLACK));

    assert_eq!(board.pieces[board.king_sq[pieces::WHITE_S] as usize], pieces::WK);
    assert_eq!(board.pieces[board.king_sq[pieces::BLACK_S] as usize], pieces::BK);
    true
}

#[cfg(test)]
mod test {
    use crate::Board;
    use crate::constants::{pieces, sqs};
    use crate::constants::pieces::{BLACK_S, EMPTY, WHITE, WHITE_S};
    use crate::constants::sqs::{NO_SQ, OFFBOARD};
    use crate::utils::square_utils::fr2sq;

    #[test]
    fn test_fr2sq() {
        let square = fr2sq(3, 5);
        assert_eq!(square,
                   74,
                   "Did not convert file and rank into correct square"
        );
    }

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
        let start: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq e3 1 2";
        let white_pawn_bb: u64 = 0x0000_0000_0000_FF00;

        let mut board = Board::new();
        unsafe { board.parse_fen(start) };
        assert_eq!(board.pieces[23], pieces::WB as u8, "Did not correctly place white bishop on F1");
        assert_eq!(board.pieces[sqs::F3 as usize], pieces::WN as u8, "Did not correctly place white knight on F3");
        assert_eq!(board.pieces[0], sqs::OFFBOARD, "Did not preserve offboard values");
        assert_eq!(board.side, 1, "Did not correctly set it as black's move");
        assert_eq!(board.castle_perm, 7, "Did not correctly set castling permission");
        assert_eq!(board.en_passant, sqs::E3 as u8, "Did not correctly set en passant square");
        //assert_eq!(board.)
        //assert_eq!(board.pawns[0].board, white_pawn_bb, "Did not correctly set position of white pawn bitboard");
    }

    #[test]
    fn test_reset_position() {
        let start: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut board = Board::new();
        unsafe { board.parse_fen(start) };
        board.update_material_list();
        board.reset_position();

        for piece in board.pieces {
            assert!(piece == OFFBOARD || piece == EMPTY, "Did not reset all squares to offboard");
        }

        for bitboard in board.bitboards {
            assert_eq!(bitboard.board, 0, "Did not reset bitboards to 0");
        }

        assert_eq!(board.king_sq[WHITE_S], 0, "Did not reset white king square");
        assert_eq!(board.king_sq[BLACK_S], 0, "Did not reset white king square");

        assert_eq!(board.fifty_move, 0, "Did not reset fifty moves");
        assert_eq!(board.side, 2, "Did not reset side");
        assert_eq!(board.en_passant, NO_SQ, "Did not reset en_passant");
        assert_eq!(board.ply, 0, "Did not reset ply");
        assert_eq!(board.castle_perm, 0, "Did not reset castle_perm");
        //TODO: Finish
    }

    #[test]
    fn test_update_material_list() {
        const RANK_2: u64 = 0x0000_0000_0000_FF00;
        const RANK_7: u64 = 0x00FF_0000_0000_0000;
        let both_ranks = RANK_2 | RANK_7;
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
        assert_eq!(format!("{:b}",board.bitboards[pieces::WHITE as usize].board), format!("{:b}", RANK_2), "Did not set white bitboard correctly");
        assert_eq!(format!("{:b}",board.bitboards[pieces::BLACK as usize].board), format!("{:b}", RANK_7), "Did not set black bitboard correctly");
        assert_eq!(format!("{:b}",board.bitboards[pieces::BOTH as usize].board), format!("{:b}", both_ranks), "Did not set both bitboard correctly");
    }
}

