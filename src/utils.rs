pub mod utils {
    use crate::bitboard::bitboard::BitBoard;
    use crate::Board;
    use crate::constants::{piece_values, pieces, ranks, squares};

    pub fn fr2sq(file: u8, rank: u8) -> u8 {
        (21 + file) + (rank * 10)
    }

    /// Giant function to check if there is anything obviously wrong with the board's state
    /// TODO: need to add an assert for a valid hash key
    /// # Panic
    /// Will panic if any of the board's state seems to be incorrect
    pub fn check_board(board:Board) -> bool {
        let mut num_pieces = [0; 13];
        let mut big_pieces = [0; 2];
        let mut major_pieces = [0; 2];
        let mut minor_pieces = [0; 2];
        let mut material = [0; 2];

        let mut pawns = [BitBoard::new(0); 3];

        pawns[piece_values::WHITE_S] = board.pawns[piece_values::WHITE_S];
        pawns[piece_values::BLACK_S] = board.pawns[piece_values::BLACK_S];
        pawns[piece_values::BOTH_S] = board.pawns[piece_values::BOTH_S];

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
                let color = piece_values::PIECE_COLOR[piece] as usize;
                if piece_values::BIG_PIECE[piece] { big_pieces[color] += 1; }
                if piece_values::MAJOR_PIECE[piece] { major_pieces[color] += 1; }
                if piece_values::MINOR_PIECE[piece] { minor_pieces[color] += 1; }

                material[color] += piece_values::VALUE[piece];
            }
        }
        for piece in pieces::WP as usize..=pieces::BK as usize {
            assert_eq!(num_pieces[piece] as u8, board.num_pieces[piece]);
        }
        // check bitboards

        let mut pcount = pawns[piece_values::WHITE_S].count_bits();
        assert_eq!(pcount, board.num_pieces[pieces::WP as usize]);
        pcount = pawns[piece_values::BLACK_S].count_bits();
        assert_eq!(pcount, board.num_pieces[pieces::BP as usize]);
        pcount = pawns[piece_values::BOTH_S].count_bits();
        assert_eq!(pcount, board.num_pieces[pieces::BP as usize] + board.num_pieces[pieces::WP as usize]);

        // check bitboards squares

        while pawns[piece_values::WHITE_S].board > 0 {
            let sq64 = pawns[piece_values::WHITE_S].pop_bit();
            assert_eq!(board.pieces[board.sq64_to_sq120[sq64 as usize] as usize], pieces::WP);
        }

        while pawns[piece_values::BLACK_S].board > 0 {
            let sq64 = pawns[piece_values::BLACK_S].pop_bit();
            assert_eq!(board.pieces[board.sq64_to_sq120[sq64 as usize] as usize], pieces::BP);
        }

        while pawns[piece_values::BOTH_S].board > 0 {
            let sq64 = pawns[piece_values::BOTH_S].pop_bit();
            assert!(board.pieces[board.sq64_to_sq120[sq64 as usize] as usize] == pieces::BP ||
                board.pieces[board.sq64_to_sq120[sq64 as usize] as usize] == pieces::WP);
        }

        assert!(material[piece_values::WHITE_S] == board.material[piece_values::WHITE_S] &&
            material[piece_values::BLACK_S] == board.material[piece_values::BLACK_S]);
        assert!(minor_pieces[piece_values::WHITE_S] == board.num_minor_pieces[piece_values::WHITE_S] &&
            minor_pieces[piece_values::BLACK_S] == board.num_minor_pieces[piece_values::BLACK_S]);
        assert!(major_pieces[piece_values::WHITE_S] == board.num_major_pieces[piece_values::WHITE_S] &&
            major_pieces[piece_values::BLACK_S] == board.num_major_pieces[piece_values::BLACK_S]);
        assert!(big_pieces[piece_values::WHITE_S] == board.num_big_pieces[piece_values::WHITE_S] &&
            big_pieces[piece_values::BLACK_S] == board.num_big_pieces[piece_values::BLACK_S]);

        assert!(board.side == piece_values::WHITE || board.side == piece_values::BLACK);

        assert!(board.en_passant == squares::NO_SQ ||
            ( board.ranks_squares[board.en_passant as usize] == ranks::RANK_6 &&
                board.side == piece_values::WHITE) ||
            (board.ranks_squares[board.en_passant as usize] == ranks::RANK_3 &&
                board.side == piece_values::BLACK));

        assert_eq!(board.pieces[board.king_sq[piece_values::WHITE_S] as usize], pieces::WK);
        assert_eq!(board.pieces[board.king_sq[piece_values::BLACK_S] as usize], pieces::BK);
        true
    }
}

