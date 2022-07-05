use crate::constants::pieces::{BIG_PIECE, BOTH, BP, EMPTY, MAJOR_PIECE, PIECE_COLOR, VALUE, WHITE, WK_CASTLE, WP};
use crate::constants::squares::{A1, A8, C1, C8, D1, D8, F1, F8, G1, G8, H1, H8, NO_SQ};
use crate::constants::{pieces, squares};
use crate::game_board::bitboard::BitBoard;
use crate::moves::gamemove::{GameMove, MFLAG_EP, MFLAG_PS};
use crate::moves::movegen::square_is_attacked;
use crate::moves::validate::is_sq_on_board;
use crate::utils::hashkeys::BoardHasher;
use crate::utils::piece_utils::{piece_is_king, piece_is_pawn};
use crate::utils::square_utils::fr2sq;
//Piece list isn't getting updated right?

/// Code used for storing the general state of the board

const CASTLE_PERM: [u8; 120] = [
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 13, 15, 15,
    15, 12, 15, 15, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 7, 15, 15, 15, 3,
    15, 15, 11, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
]; // For bitwise and with castle perm because castle perm is kept as 4 bits

#[derive(Debug, Copy, Clone)]
pub struct PastMove {
    game_move: GameMove,
    en_passant: u8,
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

    pos_key: u64,

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
        for rank in squares::RANK_1..squares::RANK_NONE {
            for file in squares::FILE_A..squares::FILE_NONE {
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
            pos_key: 0,
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

    #[inline(always)]
    pub fn sq64(&self, sq120: u8) -> u8 {
        self.sq120_to_sq64[sq120 as usize]
    }

    #[inline(always)]
    pub fn sq120(&self, sq64: u8) -> u8 {
        self.sq64_to_sq120[sq64 as usize]
    }

    /// Resets the position to an empty board
    pub fn reset_position(&mut self) {
        for i in 0..120 {
            self.pieces[i] = squares::OFFBOARD;
        }
        for i in 0..64 {
            self.pieces[usize::try_from(self.sq120(i)).unwrap()] = pieces::EMPTY;
        }

        for i in 0..2 {
            self.num_big_pieces[i] = 0;
            self.num_major_pieces[i] = 0;
            self.num_minor_pieces[i] = 0;
            self.material[i] = 0;
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
        self.en_passant = squares::NO_SQ;
        self.fifty_move = 0;

        self.ply = 0;
        self.history_ply = 0;

        self.castle_perm = 0;

        self.pos_key = 0;
    }

    /// Updates the rest of the board's state with regards to pieces to match the current piece list
    pub fn update_material_list(&mut self) {
        let mut sq: u8;
        for i in 0..120 {
            sq = i as u8;
            let piece = self.pieces[i];
            let color: usize;
            if piece != squares::OFFBOARD && piece != pieces::EMPTY {
                color = pieces::PIECE_COLOR[piece as usize] as usize;

                if pieces::BIG_PIECE[piece as usize] {
                    self.num_big_pieces[color] += 1;
                }
                if pieces::MINOR_PIECE[piece as usize] {
                    self.num_minor_pieces[color] += 1;
                }
                if pieces::MAJOR_PIECE[piece as usize] {
                    self.num_major_pieces[color] += 1;
                }

                self.material[color] += pieces::VALUE[piece as usize];

                self.piece_list[piece as usize][self.num_pieces[piece as usize] as usize] = sq;
                self.num_pieces[piece as usize] += 1;

                if piece == pieces::WK || piece == pieces::BK {
                    self.king_sq[color] = sq;
                }

                if piece == pieces::WP || piece == pieces::BP {
                    self.bitboards[color].set_bit(self.sq64(sq));
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
        let mut rank: i32 = squares::RANK_8 as i32;
        let mut file: i32 = squares::FILE_A as i32;

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
                }
                '/' | ' ' => {
                    rank -= 1;
                    file = squares::FILE_A as i32;
                    c = c.add(1);
                    continue;
                }
                _ => panic!("Inorrectly formatted string"),
            }

            // Set the square for the piece that was found or skip forward to the position
            // of the next piece
            for _ in 0..count {
                sq64 = rank * 8 + file;
                sq120 = self.sq64_to_sq120[sq64 as usize];
                if piece != pieces::EMPTY {
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

            if *c as char == ' ' {
                break;
            }

            match *c as char {
                'K' => self.castle_perm |= pieces::WK_CASTLE,
                'Q' => self.castle_perm |= pieces::WQ_CASTLE,
                'k' => self.castle_perm |= pieces::BK_CASTLE,
                'q' => self.castle_perm |= pieces::BQ_CASTLE,
                '-' => self.castle_perm = 0,
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
                _ => panic!("Incorrectly formatted string"),
            }
            let rank_char: char = *c.add(1) as char;
            rank = rank_char.to_digit(10).unwrap() as i32 - 1; // Rank in string 1 index, so subtract 1
            self.en_passant = fr2sq(file as u8, rank as u8);
        }

        //self.ply =

        self.pos_key =
            self.hasher
                .generate_key(self.pieces, self.side, self.en_passant, self.castle_perm);
    }

    // Functions for making a move on the board

    #[inline(always)]
    pub fn hash_piece(&mut self, pce: u8, sq: u8) {
        self.pos_key ^= self.hasher.piece_keys[pce as usize][sq as usize];
    }

    #[inline(always)]
    pub fn hash_castle(&mut self) {
        self.pos_key ^= self.hasher.castle_keys[self.castle_perm as usize];
    }

    #[inline(always)]
    pub fn hash_side(&mut self) {
        self.pos_key ^= self.hasher.side_key;
    }

    #[inline(always)]
    pub fn hash_en_passant(&mut self) {
        self.pos_key ^= self.hasher.piece_keys[EMPTY as usize][self.en_passant as usize];
    }

    /// Removes a piece from the board state
    ///
    /// # Arguments
    ///
    /// * `pos`: The board state
    /// * `sq`: The square the piece is sitting on. This should be a 120 square board number
    ///
    /// returns: ()
    #[inline]
    fn clear_piece(&mut self, sq: u8) {
        let pce = self.pieces[sq as usize] as usize;
        let col = PIECE_COLOR[pce as usize] as usize;
        let mut t_pce_num = -1;

        self.hash_piece(self.pieces[sq as usize], sq);

        self.pieces[sq as usize] = EMPTY;
        self.material[col] -= VALUE[pce];

        if BIG_PIECE[pce] {
            self.num_big_pieces[col] -= 1;
            if MAJOR_PIECE[pce] {
                self.num_major_pieces[col] -= 1;
            } else {
                self.num_minor_pieces[col] -= 1;
            }
        } else {
            self.bitboards[col].clear_bit(self.sq64(sq));
            self.bitboards[BOTH as usize].clear_bit(self.sq64(sq));
        }

        for i in 0..self.num_pieces[pce] as usize {
            if self.piece_list[pce][i] == sq {
                t_pce_num = i as i16;
                break;
            }
        }

        if t_pce_num < 0 {
            panic!("Didn't find the right piece at the square");
        }

        self.num_pieces[pce] -= 1;
        self.piece_list[pce][t_pce_num as usize] =
            self.piece_list[pce][self.num_pieces[pce] as usize]; // Replace the removed piece with the last piece in the list, after decrementing the max index
    }

    /// Adds a piece to the board state
    ///
    /// # Arguments
    ///
    /// * `sq`: The 120-board square number for the location to add the piece to
    /// * `pce`: The piece number
    ///
    /// returns: ()
    ///
    #[inline]
    pub fn add_piece(&mut self, sq: u8, pce: u8) {
        let col = PIECE_COLOR[pce as usize] as usize;
        self.hash_piece(pce, sq);

        self.pieces[sq as usize] = pce;

        let t_pce = pce as usize;

        if BIG_PIECE[t_pce] {
            self.num_big_pieces[col] += 1;
            if MAJOR_PIECE[t_pce] {
                self.num_major_pieces[col] += 1;
            } else {
                self.num_minor_pieces[col] += 1;
            }
        } else {
            self.bitboards[col].set_bit(self.sq64(sq));
            self.bitboards[BOTH as usize].set_bit(self.sq64(sq));
        }
        self.material[col] += VALUE[t_pce];
        self.piece_list[t_pce][self.num_pieces[t_pce] as usize] = sq;
        self.num_pieces[t_pce] += 1;
    }

    /// Moves a piece from one square on the board to another square on the board.
    /// The function checks to make sure the squares are on the board, but otherwise
    /// makes no other checks to ensure the move is valid.
    ///
    /// # Arguments
    ///
    /// * `from`: The square the piece to be moved is currently on. Should be a 120 sq board number
    /// * `to`: The square to move the piece to. Should be a 120 sq board number.
    ///
    /// returns: ()
    ///
    #[inline]
    pub fn move_piece(&mut self, from: u8, to: u8) {
        if !is_sq_on_board(from) || !is_sq_on_board(to) {
            panic!("Squares are off the board");
        }
        let from_idx = from as usize;
        let to_idx = to as usize;
        let pce = self.pieces[from_idx];
        let col = PIECE_COLOR[pce as usize] as usize;

        self.hash_piece(pce, from);
        self.pieces[from_idx] = EMPTY;

        self.hash_piece(pce, to);
        self.pieces[to_idx] = pce;

        if !BIG_PIECE[pce as usize] {
            self.bitboards[col].move_bit(self.sq64(from), self.sq64(to));
            self.bitboards[BOTH as usize].move_bit(self.sq64(from), self.sq64(to));
        }

        for pces in self.piece_list.iter_mut() {
            for pce in pces.iter_mut() {
                if *pce == from {
                    *pce = to;
                    break;
                }
            }
        }
    }

    #[inline]
    pub fn undo_move(&mut self) {
        self.history_ply -= 1;
        self.ply -= 1;

        let past_move;
        match self.history.pop() {
            Some(m) => past_move = m,
            None => panic!("Undo was called with no previous moves"),
        }

        let from = past_move.game_move.origin();
        let to = past_move.game_move.destination();

        if self.en_passant != NO_SQ {
            self.hash_en_passant();
        }
        self.hash_castle();

        self.castle_perm = past_move.castle_perm;
        self.fifty_move = past_move.fifty_move;
        self.en_passant = past_move.en_passant;

        if self.en_passant != NO_SQ {
            self.hash_en_passant();
        }
        self.hash_castle();

        self.side ^= 1;
        self.hash_side();

        if past_move.game_move.is_en_passant() {
            if self.side == WHITE {
                self.add_piece(to - 10, BP);
            } else {
                self.add_piece(to + 10, WP);
            }
        } else if past_move.game_move.is_castle_move() {
            match to {
                C1 => self.move_piece(D1, A1),
                G1 => self.move_piece(F1, H1),
                C8 => self.move_piece(D8, A8),
                G8 => self.move_piece(F8, H8),
                _ => panic!("Invalid castling move"),
            }
        }

        self.move_piece(to, from);

        if piece_is_king(self.pieces[from as usize]) {
            self.king_sq[self.side as usize] = from;
        }

        let captured = past_move.game_move.capture();
        if captured != EMPTY {
            self.add_piece(to, captured);
        }

        let promoted = past_move.game_move.promoted_piece();

        if promoted != EMPTY {
            self.clear_piece(from);
            self.add_piece(
                from,
                if PIECE_COLOR[promoted as usize] == WHITE {
                    WP
                } else {
                    BP
                },
            );
        }
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `mov`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    #[inline]
    pub fn make_move(&mut self, mov: GameMove) -> bool {
        let from = mov.origin();
        let to = mov.destination();
        let side = self.side;

        let his_pos_key = self.pos_key;

        // Handle en passant capture
        if mov.is_en_passant() {
            if side == WHITE {
                self.clear_piece(to - 10);
            } else {
                self.clear_piece(to + 10);
            }
        } else if mov.is_castle_move() {
            match to {
                C1 => self.move_piece(A1, D1), // Moving the rook as part of castling
                C8 => self.move_piece(A8, D8),
                G1 => self.move_piece(H1, F1),
                G8 => self.move_piece(H8, F8),
                _ => panic!("Invalid castling move"),
            }
        }

        if self.en_passant != NO_SQ {
            self.hash_en_passant();
        }

        self.hash_castle();

        self.history.push(PastMove {
            pos_key: his_pos_key,
            game_move: mov,
            fifty_move: self.fifty_move,
            en_passant: self.en_passant,
            castle_perm: self.castle_perm,
        });

        self.castle_perm &= CASTLE_PERM[from as usize];
        self.castle_perm &= CASTLE_PERM[to as usize];
        self.en_passant = NO_SQ;

        self.hash_castle();

        let captured = mov.capture();

        if captured != EMPTY {
            self.clear_piece(to);
            self.fifty_move = 0;
        } else {
            self.fifty_move += 1;
        }

        self.history_ply += 1;
        self.ply += 1;

        if piece_is_pawn(self.pieces[from as usize]) {
            if mov.is_pawn_start() {
                if side == WHITE {
                    self.en_passant = from + 10;
                } else {
                    self.en_passant = from - 10;
                }
                self.hash_en_passant();
            }
        }

        self.move_piece(from, to);

        let promoted_pce = mov.promoted_piece();

        if promoted_pce != EMPTY {
            self.clear_piece(to);
            self.add_piece(to, promoted_pce);
        }

        if piece_is_king(self.pieces[to as usize]) {
            self.king_sq[self.side as usize] = to;
        }

        self.side ^= 1;
        self.hash_side();

        if square_is_attacked(self.king_sq[side as usize], self.side, &self.pieces) {
            self.undo_move();
            return false;
        }

        true
    }
}

/// Prints the board
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const PIECE_CHARS: [&str; 13] = [
            " . ", " P ", " N ", " B ", " R ", " Q ", " K ", " p ", " n ", " b ", " r ", " q ",
            " k ",
        ];
        const SIDE_CHARS: [char; 3] = ['w', 'b', '-'];
        const RANK_CHARS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
        const FILE_CHARS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut output = String::from("");
        for rank in (squares::RANK_1..=squares::RANK_8).rev() {
            for file in squares::FILE_A..squares::FILE_H + 1 {
                let sq = fr2sq(file, rank);
                let piece = self.pieces[sq as usize];
                output.push_str(PIECE_CHARS[piece as usize]);
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

/// Giant function to check if there is anything obviously wrong with the board's state
/// TODO: need to add an assert for a valid hash key
/// # Panic
/// Will panic if any of the board's state seems to be incorrect
pub fn check_board(board: &Board) -> bool {
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
        let sq120 = board.sq120(sq64) as usize;
        let piece = board.pieces[sq120] as usize;
        if piece as u8 != pieces::EMPTY {
            num_pieces[piece] += 1;
            let color = pieces::PIECE_COLOR[piece] as usize;
            if pieces::BIG_PIECE[piece] {
                big_pieces[color] += 1;
            }
            if pieces::MAJOR_PIECE[piece] {
                major_pieces[color] += 1;
            }
            if pieces::MINOR_PIECE[piece] {
                minor_pieces[color] += 1;
            }

            material[color] += pieces::VALUE[piece];
        }
    }

    // check bitboards

    let mut pcount = pawns[pieces::WHITE_S].count_bits();
    assert_eq!(pcount, board.num_pieces[pieces::WP as usize]);
    pcount = pawns[pieces::BLACK_S].count_bits();
    assert_eq!(pcount, board.num_pieces[pieces::BP as usize]);
    pcount = pawns[pieces::BOTH_S].count_bits();
    assert_eq!(
        pcount,
        board.num_pieces[pieces::BP as usize] + board.num_pieces[pieces::WP as usize]
    );

    // check bitboards squares

    while pawns[pieces::WHITE_S].board > 0 {
        let sq64 = pawns[pieces::WHITE_S].pop_bit();
        assert_eq!(board.pieces[board.sq120(sq64) as usize], pieces::WP);
    }

    while pawns[pieces::BLACK_S].board > 0 {
        let sq64 = pawns[pieces::BLACK_S].pop_bit();
        assert_eq!(board.pieces[board.sq120(sq64) as usize], pieces::BP);
    }

    while pawns[pieces::BOTH_S].board > 0 {
        let sq64 = pawns[pieces::BOTH_S].pop_bit();
        assert!(
            board.pieces[board.sq120(sq64) as usize] == pieces::BP
                || board.pieces[board.sq120(sq64) as usize] == pieces::WP
        );
    }

    assert!(
        material[pieces::WHITE_S] == board.material[pieces::WHITE_S]
            && material[pieces::BLACK_S] == board.material[pieces::BLACK_S]
    );
    assert!(
        minor_pieces[pieces::WHITE_S] == board.num_minor_pieces[pieces::WHITE_S]
            && minor_pieces[pieces::BLACK_S] == board.num_minor_pieces[pieces::BLACK_S]
    );
    assert!(
        major_pieces[pieces::WHITE_S] == board.num_major_pieces[pieces::WHITE_S]
            && major_pieces[pieces::BLACK_S] == board.num_major_pieces[pieces::BLACK_S]
    );
    assert!(
        big_pieces[pieces::WHITE_S] == board.num_big_pieces[pieces::WHITE_S]
            && big_pieces[pieces::BLACK_S] == board.num_big_pieces[pieces::BLACK_S]
    );

    assert!(board.side == pieces::WHITE || board.side == pieces::BLACK);

    // assert!(board.en_passant == squares::NO_SQ ||
    //     ( board.ranks_squares[board.en_passant as usize] == squares::RANK_6 &&
    //         board.side == pieces::WHITE) ||
    //     (board.ranks_squares[board.en_passant as usize] == squares::RANK_3 &&
    //         board.side == pieces::BLACK));

    assert_eq!(
        board.pieces[board.king_sq[pieces::WHITE_S] as usize],
        pieces::WK
    );
    assert_eq!(
        board.pieces[board.king_sq[pieces::BLACK_S] as usize],
        pieces::BK
    );
    true
}

#[cfg(test)]
mod test {
    use crate::constants::pieces::{
        BLACK, BLACK_S, BP, BQ_CASTLE, BR, EMPTY, WHITE, WHITE_S, WK_CASTLE, WP, WQ, WQ_CASTLE, WR,
    };
    use crate::constants::squares::{A2, A3, A4, FILE_B, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, NO_SQ, OFFBOARD, RANK_1, RANK_2, RANK_3, RANK_5, RANK_6, RANK_7, RANK_8};
    use crate::constants::{pieces, squares};
    use crate::game_board::board::PastMove;
    use crate::game_board::board::{check_board, Board, GameMove};
    use crate::moves::validate::is_sq_on_board;
    use crate::utils::square_utils::fr2sq;

    #[test]
    fn test_fr2sq() {
        let square = fr2sq(3, 5);
        assert_eq!(
            square, 74,
            "Did not convert file and rank into correct square"
        );
    }

    #[test]
    fn test_parse_fen() {
        let start: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq e3 1 2";
        let white_pawn_bb: u64 = 0x0000_0000_0000_FF00;

        let mut board = Board::new();
        unsafe { board.parse_fen(start) };
        assert_eq!(
            board.pieces[23],
            pieces::WB as u8,
            "Did not correctly place white bishop on F1"
        );
        assert_eq!(
            board.pieces[squares::F3 as usize],
            pieces::WN as u8,
            "Did not correctly place white knight on F3"
        );
        assert_eq!(
            board.pieces[0],
            squares::OFFBOARD,
            "Did not preserve offboard values"
        );
        assert_eq!(board.side, 1, "Did not correctly set it as black's move");
        assert_eq!(
            board.castle_perm, 15,
            "Did not correctly set castling permission"
        );
        assert_eq!(
            board.en_passant,
            squares::E3 as u8,
            "Did not correctly set en passant square"
        );
        //assert_eq!(board.)
        //assert_eq!(board.pawns[0].board, white_pawn_bb, "Did not correctly set position of white pawn bitboard");
    }

    #[test]
    fn test_parse_fen_castling() {
        let mut board = Board::new();
        unsafe {
            board.parse_fen("4k2r/8/8/8/8/8/8/4K3 w k - 0 1");
        }
        board.update_material_list();
        assert_eq!(
            board.castle_perm & WK_CASTLE,
            0,
            "Incorrectly has white king castling permission"
        );
        assert_eq!(
            board.castle_perm & WQ_CASTLE,
            0,
            "Incorrectly has white queen castling permission"
        );
        assert_eq!(
            board.castle_perm & BQ_CASTLE,
            0,
            "Incorrectly has black queen castling permission"
        );
    }

    #[test]
    fn test_reset_position() {
        let start: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut board = Board::new();
        unsafe { board.parse_fen(start) };
        board.update_material_list();
        board.reset_position();

        for piece in board.pieces {
            assert!(
                piece == OFFBOARD || piece == EMPTY,
                "Did not reset all squares to offboard"
            );
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
        assert_eq!(
            board.num_major_pieces[0], 4,
            "Did not update with correct number of white major pieces"
        );
        assert_eq!(
            board.num_major_pieces[1], 4,
            "Did not update with correct number of black major pieces"
        );
        assert_eq!(
            board.num_minor_pieces[0], 4,
            "Did not update with correct number of white minor pieces"
        );
        assert_eq!(
            board.num_minor_pieces[1], 4,
            "Did not update with correct number of black minor pieces"
        );
        assert_eq!(
            board.num_big_pieces[0], 8,
            "Did not update with correct number of white big pieces"
        );
        assert_eq!(
            board.num_big_pieces[1], 8,
            "Did not update with correct number of black big pieces"
        );
        assert_eq!(
            board.material[0], 54200,
            "Did not correctly set material value for white"
        );
        assert_eq!(
            board.material[1], 54200,
            "Did not correctly set material value for black"
        );
        assert_eq!(
            board.piece_list[1][4], 35,
            "Did not correctly set square for white pawn"
        );
        assert_eq!(
            format!("{:b}", board.bitboards[pieces::WHITE as usize].board),
            format!("{:b}", RANK_2),
            "Did not set white bitboard correctly"
        );
        assert_eq!(
            format!("{:b}", board.bitboards[pieces::BLACK as usize].board),
            format!("{:b}", RANK_7),
            "Did not set black bitboard correctly"
        );
        assert_eq!(
            format!("{:b}", board.bitboards[pieces::BOTH as usize].board),
            format!("{:b}", both_ranks),
            "Did not set both bitboard correctly"
        );
    }

    #[test]
    fn test_clear_piece_rook() {
        let fen = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board = Board::new();
        unsafe {
            board.parse_fen(fen);
        }
        board.update_material_list();
        let before = board.clone();
        let sq = board.sq120(0);
        board.clear_piece(sq);
        assert_eq!(
            before.num_major_pieces[WHITE as usize] - 1,
            board.num_major_pieces[WHITE as usize],
            "Did not decrement number of major pieces"
        );
        assert_eq!(
            before.num_big_pieces[WHITE as usize] - 1,
            board.num_big_pieces[WHITE as usize],
            "Did not decrement number of big pieces"
        );
        assert_eq!(
            before.num_pieces[WR as usize] - 1,
            board.num_pieces[WR as usize],
            "Did not decrement number of rooks"
        );
        assert_eq!(
            before.num_major_pieces[BLACK as usize], board.num_major_pieces[BLACK as usize],
            "Decremented number of major pieces for wrong color"
        );
        assert_eq!(
            before.num_minor_pieces[WHITE as usize], board.num_minor_pieces[WHITE as usize],
            "Decremented wrong category of piece"
        );
        assert_eq!(
            before.bitboards[WHITE as usize].board, board.bitboards[WHITE as usize].board,
            "Changed bitboard when piece wasn't a pawn"
        );
        assert_eq!(
            before.piece_list[WR as usize][1], board.piece_list[WR as usize][0],
            "Did not shorten piece list correctly"
        );
    }

    #[test]
    fn test_clear_piece_pawn() {
        let fen1 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P3/RNBQKBNR w KQkq e6 0 10";
        let fen2 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board1 = Board::new();
        let mut board2 = Board::new();
        unsafe {
            board1.parse_fen(fen1);
        }
        unsafe {
            board2.parse_fen(fen2);
        }
        board1.update_material_list();
        board2.update_material_list();
        let sq = board1.sq120(8);
        board1.clear_piece(sq);
        assert_eq!(
            board1.bitboards, board2.bitboards,
            "Did not remove pawn from bitboards"
        );
        assert_eq!(
            board1.num_pieces, board2.num_pieces,
            "Did not update number of pieces"
        );
        assert_eq!(
            board1.piece_list[WP as usize][0], board2.piece_list[WP as usize][6],
            "Did not update piece list correctly"
        );
    }

    #[test]
    fn test_add_pawn() {
        let fen1 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/P1P1P3/RNBQKBNR w KQkq e6 0 10";
        let fen2 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board1 = Board::new();
        let mut board2 = Board::new();
        unsafe {
            board1.parse_fen(fen1);
        }
        unsafe {
            board2.parse_fen(fen2);
        }
        board1.update_material_list();
        board2.update_material_list();
        let sq = board1.sq120(8);
        board2.add_piece(sq, WP);
        assert_eq!(
            board1.bitboards, board2.bitboards,
            "Did not remove pawn from bitboards"
        );
        assert_eq!(
            board1.num_pieces, board2.num_pieces,
            "Did not update number of pieces"
        );
        assert_eq!(
            board1.piece_list[WP as usize][0], board2.piece_list[WP as usize][7],
            "Did not update piece list correctly"
        );
    }

    #[test]
    fn test_add_rook() {
        let fen1 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let fen2 = "1nbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board1 = Board::new();
        let mut board2 = Board::new();
        unsafe {
            board1.parse_fen(fen1);
        }
        unsafe {
            board2.parse_fen(fen2);
        }
        board1.update_material_list();
        board2.update_material_list();
        let sq = board1.sq120(56);
        board2.add_piece(sq, BR);
        assert_eq!(
            board1.piece_list[BR as usize][0], board2.piece_list[BR as usize][1],
            "Did not update piece list correctly"
        );
        assert_eq!(
            board1.num_pieces, board2.num_pieces,
            "Did not update number of pieces"
        );
        assert_eq!(
            board1.num_big_pieces, board2.num_big_pieces,
            "Did not update number of big pieces"
        );
        assert_eq!(
            board1.num_major_pieces, board2.num_major_pieces,
            "Did not update number of major pieces"
        );
        assert_eq!(
            board1.pieces, board2.pieces,
            "Did not update pieces correctly"
        );
    }

    #[test]
    fn test_move_rook() {
        let fen1 = "rnbqkbnr/1p1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let fen2 = "1nbqkbnr/rp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board1 = Board::new();
        let mut board2 = Board::new();
        unsafe {
            board1.parse_fen(fen1);
        }
        unsafe {
            board2.parse_fen(fen2);
        }
        board1.update_material_list();
        board2.update_material_list();
        let from = board1.sq120(56);
        let to = board1.sq120(48);
        board1.move_piece(from, to);
        assert_eq!(
            board1.pieces, board2.pieces,
            "Did not update pieces correctly"
        );
    }

    #[test]
    fn test_move_pawn() {
        let fen1 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let fen2 = "rnbqkbnr/1p1p1pPp/p7/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board1 = Board::new();
        let mut board2 = Board::new();
        unsafe {
            board1.parse_fen(fen1);
        }
        unsafe {
            board2.parse_fen(fen2);
        }
        board1.update_material_list();
        board2.update_material_list();
        let from = board1.sq120(48);
        let to = board1.sq120(40);
        board1.move_piece(from, to);
        assert_eq!(
            board1.pieces, board2.pieces,
            "Did not update pieces correctly"
        );
        assert_eq!(
            board1.bitboards, board2.bitboards,
            "Did not update bitboards correctly"
        );
    }

    #[test]
    fn test_en_passant_move() {
        let mut board = Board::new();
        unsafe { board.parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ") }
        board.update_material_list();
        let mut mov = GameMove::new(A2, A4, 0, 0, 0x80000);
        board.make_move(mov);
        assert_eq!(board.en_passant, A3);
    }

    #[test]
    fn test_make_move() {
        let fen1 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let fen2 = "rnbqkbnr/pp1p1pPp/4P3/2p5/1P1P4/3P4/2P1P3/RNBQKBNR b KQkq - 0 10";
        let mut board1 = Board::new();
        let mut board2 = Board::new();

        unsafe {
            board1.parse_fen(fen1);
        }
        unsafe {
            board2.parse_fen(fen2);
        }
        board1.update_material_list();
        board2.update_material_list();
        let from = fr2sq(FILE_F, RANK_5);
        let to = fr2sq(FILE_E, RANK_6);
        let mov = GameMove::new(from, to, 0, 0, 0x40000);

        board1.make_move(mov);
        assert_eq!(
            board1.pieces, board2.pieces,
            "Did not update pieces correctly"
        );
        assert_eq!(
            board1.bitboards, board2.bitboards,
            "Did not update bitboards correctly"
        );
        assert_eq!(board1.fifty_move, 1, "Did not update fifty move");
        assert_eq!(board1.side, BLACK, "Did not change side");
        assert_eq!(
            board1.history.pop().unwrap().game_move,
            mov,
            "Did not update move histroy"
        );
    }

    #[test]
    fn test_undo_move() {
        let fen1 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let fen2 = "rnbqkbnr/pp1p1pPp/4P3/2p5/1P1P4/3P4/2P1P3/RNBQKBNR b KQkq - 0 10";
        let mut board1 = Board::new();
        let mut board2 = Board::new();
        unsafe {
            board1.parse_fen(fen1);
        }
        unsafe {
            board2.parse_fen(fen2);
        }
        board1.update_material_list();
        board2.update_material_list();
        let from = fr2sq(FILE_F, RANK_5);
        let to = fr2sq(FILE_E, RANK_6);
        let mov = GameMove::new(from, to, 0, 0, 0x40000);
        board2.history.push(PastMove {
            game_move: mov,
            en_passant: NO_SQ,
            castle_perm: board1.castle_perm,
            fifty_move: board1.fifty_move,
            pos_key: board1.pos_key,
        });
        board2.ply = 1;
        board2.history_ply = 1;
        board2.undo_move();
        assert_eq!(
            board1.pieces, board2.pieces,
            "Did not update pieces correctly"
        );
        assert_eq!(
            board1.bitboards, board2.bitboards,
            "Did not update bitboards correctly"
        );
        assert_eq!(board2.fifty_move, 0, "Did not update fifty move");
        assert_eq!(board2.side, WHITE, "Did not change side");
    }

    #[test]
    fn test_make_move_into_check() {
        let fen1 = "rnb1kbnr/pp1p1pPp/8/2p1pPq1/1P1P4/2NP4/2P1P3/R1BQKBNR w KQkq - 2 11";
        let mov = GameMove::new(fr2sq(FILE_E, RANK_1), fr2sq(FILE_D, RANK_2), 0, 0, 0);
        let mut board1 = Board::new();
        unsafe {
            board1.parse_fen(fen1);
        }
        let board2 = board1.clone();
        board1.make_move(mov);
        assert_eq!(
            board1.pieces, board2.pieces,
            "Changed pieces with invalid move"
        );
        assert_eq!(
            board1.bitboards, board2.bitboards,
            "Changed bitboard with invalid move"
        );
        assert_eq!(
            board1.fifty_move, 0,
            "Updated fifty moves with invalid move"
        );
        assert_eq!(board1.side, WHITE, "Changed side with invalid move");
        assert_eq!(board1.history.len(), 0, "Invalid move remained in board");
    }

    fn test_make_and_undo() {
        let fen1 = "rnbqkbnr/pp1p1pPp/8/2p1pP2/1P1P4/3P4/2P1P3/RNBQKBNR w KQkq e6 0 10";
        let mut board1 = Board::new();
        unsafe {
            board1.parse_fen(fen1);
        }
        board1.update_material_list();
        let board2 = board1.clone();
        let mut moves = vec![];
        moves.push(GameMove::new(
            fr2sq(FILE_F, RANK_5),
            fr2sq(FILE_E, RANK_6),
            0,
            0,
            0x40000,
        ));
        moves.push(GameMove::new(
            fr2sq(FILE_B, RANK_7),
            fr2sq(FILE_B, RANK_5),
            0,
            0,
            0x80000,
        ));
        moves.push(GameMove::new(
            fr2sq(FILE_G, RANK_1),
            fr2sq(FILE_F, RANK_3),
            0,
            0,
            0,
        ));
        moves.push(GameMove::new(
            fr2sq(FILE_D, RANK_7),
            fr2sq(FILE_E, RANK_6),
            WP,
            0,
            0,
        ));
        moves.push(GameMove::new(
            fr2sq(FILE_F, RANK_1),
            fr2sq(FILE_H, RANK_3),
            0,
            0,
            0,
        ));
        moves.push(GameMove::new(
            fr2sq(FILE_E, RANK_8),
            fr2sq(FILE_E, RANK_7),
            0,
            0,
            0,
        ));
        moves.push(GameMove::new(
            fr2sq(FILE_E, RANK_1),
            fr2sq(FILE_G, RANK_1),
            0,
            0,
            0x1000000,
        ));
        moves.push(GameMove::new(
            fr2sq(FILE_D, RANK_8),
            fr2sq(FILE_E, RANK_8),
            0,
            0,
            0,
        ));
        moves.push(GameMove::new(
            fr2sq(FILE_G, RANK_7),
            fr2sq(FILE_H, RANK_8),
            BR,
            WQ,
            0x40000,
        ));

        for mov in &moves {
            board1.make_move(*mov);
        }
        for i in 0..moves.len() {
            board1.undo_move();
        }
        assert_eq!(
            board1.pieces, board2.pieces,
            "Did not end up with same pieces after undo moves"
        );
        assert_eq!(
            board1.bitboards, board2.bitboards,
            "Did not end up with same bitboards after undo moves"
        );
        assert_eq!(board1.fifty_move, 0, "Did not undo fifty moves");
        assert_eq!(board1.side, WHITE, "Did not undo side");
        assert_eq!(board1.history.len(), 0, "Didn't wind back history");
    }
}
