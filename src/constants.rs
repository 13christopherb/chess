pub mod castling {
    pub const WK_CASTLE:u8 = 1;
    pub const WQ_CASTLE:u8 = 2;
    pub const BK_CASTLE:u8 = 3;
    pub const BQ_CASTLE:u8 = 4;
}

pub mod pieces {
    use crate::constants::sqs::OFFBOARD;

    pub const WHITE:u8 = 0;
    pub const BLACK:u8 = 1;
    pub const BOTH:u8 = 2;
    pub const EMPTY:u8 = 0;

    pub const WP:u8 = 1;
    pub const WN:u8 = 2;
    pub const WB:u8 = 3;
    pub const WR:u8 = 4;
    pub const WQ:u8 = 5;
    pub const WK:u8 = 6;
    pub const BP:u8 = 7;
    pub const BN:u8 = 8;
    pub const BB:u8 = 9;
    pub const BR:u8 = 10;
    pub const BQ:u8 = 11;
    pub const BK:u8 = 12;

    pub const WHITE_S:usize = 0;
    pub const BLACK_S:usize = 1;
    pub const EMPTY_S:usize = 0;
    pub const BOTH_S:usize = 2;

    pub const KNIGHT_NUMBER:[bool; 13] = [false, false, true, false, false, false,
        false, false, true, false, false, false, false];
    pub const BISHOP_QUEEN_NUMBER:[bool; 13] = [false, false, false, true, false, true,
        false, false, false, true, false, true, false];
    pub const ROOK_QUEEN_NUMBER:[bool; 13] = [false, false, false, false, true, true,
        false, false, false, false, true, true, false];
    pub const KING_NUMBER:[bool; 13] = [false, false, false, false, false, false,
        true, false, false, false, false, false, true];

    #[inline(always)]
    pub fn is_knight(pce:u8) -> bool { KNIGHT_NUMBER[pce as usize] }

    #[inline(always)]
    pub fn is_bishop_or_queen(pce:u8) -> bool { BISHOP_QUEEN_NUMBER[pce as usize] }

    #[inline(always)]
    pub fn is_rook_or_queen(pce:u8) -> bool { ROOK_QUEEN_NUMBER[pce as usize] }

    #[inline(always)]
    pub fn is_king(pce:u8) -> bool { KING_NUMBER[pce as usize] }

    pub const BIG_PIECE:[bool; 13] = [ false, false, true, true, true, true, true, false, true, true, true, true, true ];
    pub const MAJOR_PIECE:[bool; 13] = [ false, false, false, false, true, true, true, false, false, false, true, true, true ];
    pub const MINOR_PIECE:[bool; 13] = [ false, false, true, true, false, false, false, false, true, true, false, false, false ];
    pub const SLIDES:[bool; 13] = [false, false, false, true, true, true, false, false, false, true, true, true, false];
    pub const LOOP_SLIDE:[u8; 8] = [WB, WR, WQ, 0, BB, BR, BQ, 0];
    pub const LOOP_SLIDE_INDEX:[u8; 2] = [0, 4];
    pub const LOOP_NONSLIDE:[u8; 6] = [WN, WK, 0, BN, BK, 0];
    pub const LOOP_NONSLIDE_INDEX:[u8; 2] = [0, 3];
    pub const VALUE:[u32; 13] = [0, 100, 325, 325,  550, 1000, 50000, 100, 325, 325, 550, 1000, 50000];
    pub const PIECE_COLOR:[u8; 13] = [BOTH, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
        BLACK, BLACK, BLACK, BLACK, BLACK, BLACK];

    pub const PIECE_DIR:[[i32; 8]; 13] = [
        [ 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ -8, -19, -21, -12, 8, 19, 21, 12 ],
        [-9, -11, 11, 9, 0, 0, 0, 0 ],
        [-1, -10, 1, 10, 0, 0, 0, 0 ],
        [-1, -10, 1, 10, -9, -11, 11, 9],
        [-1, -10, 1, 10, -9, -11, 11, 9],
        [ 0, 0, 0, 0, 0, 0, 0, 0],
        [-8, -19, -21, -12, 8, 19, 21, 12],
        [-9, -11, 11, 9, 0, 0, 0, 0],
        [-1, -10, 1, 10, 0, 0, 0, 0],
        [-1, 10, 1, 10, -9, -11, 11, 9],
        [-1, -10, 1, 10, -9, -11, 11, 9]
    ];

    pub const NUM_DIR:[usize; 13] = [0, 0, 8, 4, 4, 8, 8, 0, 8, 4, 4, 8, 8];

    #[inline(always)]
    pub fn is_same_color(pce:u8, color:u8) -> bool { PIECE_COLOR[pce as usize] == color }

    #[inline(always)]
    pub fn is_white(pce:u8,) -> bool {pce != OFFBOARD && PIECE_COLOR[pce as usize] == WHITE }

    #[inline(always)]
    pub fn is_black(pce:u8,) -> bool {pce != OFFBOARD && PIECE_COLOR[pce as usize] == BLACK }

}

pub mod files {
    use crate::constants::sqs::OFFBOARD;

    pub const FILE_A:u8 = 0;
    pub const FILE_B:u8 = 1;
    pub const FILE_C:u8 = 2;
    pub const FILE_D:u8 = 3;
    pub const FILE_E:u8 = 4;
    pub const FILE_F:u8 = 5;
    pub const FILE_G:u8 = 6;
    pub const FILE_H:u8 = 7;
    pub const FILE_NONE:u8 = 8;

    pub const FILE_SQUARES:[u8; 120] = [
        OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
        OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
        OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
        OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
        OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
        OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
        OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
        OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
        OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
        OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
        OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
        OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
    ];
}

pub mod ranks {
    use crate::constants::sqs::OFFBOARD;

    pub const RANK_1:u8 = 0;
    pub const RANK_2:u8 = 1;
    pub const RANK_3:u8 = 2;
    pub const RANK_4:u8 = 3;
    pub const RANK_5:u8 = 4;
    pub const RANK_6:u8 = 5;
    pub const RANK_7:u8 = 6;
    pub const RANK_8:u8 = 7;
    pub const RANK_NONE:u8 = 8;

    pub const RANK_SQUARES:[u8; 120] = [
        OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
        OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
        OFFBOARD, RANK_1, RANK_1, RANK_1, RANK_1, RANK_1, RANK_1, RANK_1, RANK_1, OFFBOARD,
        OFFBOARD, RANK_2, RANK_2, RANK_2, RANK_2, RANK_2, RANK_2, RANK_2, RANK_2, OFFBOARD,
        OFFBOARD, RANK_3, RANK_3, RANK_3, RANK_3, RANK_3, RANK_3, RANK_3, RANK_3, OFFBOARD,
        OFFBOARD, RANK_4, RANK_4, RANK_4, RANK_4, RANK_4, RANK_4, RANK_4, RANK_4, OFFBOARD,
        OFFBOARD, RANK_5, RANK_5, RANK_5, RANK_5, RANK_5, RANK_5, RANK_5, RANK_5, OFFBOARD,
        OFFBOARD, RANK_6, RANK_6, RANK_6, RANK_6, RANK_6, RANK_6, RANK_6, RANK_6, OFFBOARD,
        OFFBOARD, RANK_7, RANK_7, RANK_7, RANK_7, RANK_7, RANK_7, RANK_7, RANK_7, OFFBOARD,
        OFFBOARD, RANK_8, RANK_8, RANK_8, RANK_8, RANK_8, RANK_8, RANK_8, RANK_8, OFFBOARD,
        OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
        OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
    ];
}

pub mod sqs {
    use crate::constants::files::FILE_SQUARES;

    pub const FILE_A:u8 = 0;
    pub const FILE_B:u8 = 1;
    pub const FILE_C:u8 = 2;
    pub const FILE_D:u8 = 3;
    pub const FILE_E:u8 = 4;
    pub const FILE_F:u8 = 5;
    pub const FILE_G:u8 = 6;
    pub const FILE_H:u8 = 7;
    pub const FILE_NONE:u8 = 8;

    pub const RANK_1:u8 = 0;
    pub const RANK_2:u8 = 1;
    pub const RANK_3:u8 = 2;
    pub const RANK_4:u8 = 3;
    pub const RANK_5:u8 = 4;
    pub const RANK_6:u8 = 5;
    pub const RANK_7:u8 = 6;
    pub const RANK_8:u8 = 7;
    pub const RANK_NONE:u8 = 8;

    pub const A1:usize = 21; pub const A2:usize = 31; pub const A3:usize = 41; pub const A4:usize = 51;
    pub const B1:usize = 22; pub const B2:usize = 32; pub const B3:usize = 42; pub const B4:usize = 52;
    pub const C1:usize = 23; pub const C2:usize = 33; pub const C3:usize = 43; pub const C4:usize = 53;
    pub const D1:usize = 24; pub const D2:usize = 34; pub const D3:usize = 44; pub const D4:usize = 54;
    pub const E1:usize = 25; pub const E2:usize = 35; pub const E3:usize = 45; pub const E4:usize = 55;
    pub const F1:usize = 26; pub const F2:usize = 36; pub const F3:usize = 46; pub const F4:usize = 56;
    pub const G1:usize = 27; pub const G2:usize = 37; pub const G3:usize = 47; pub const G4:usize = 57;
    pub const H1:usize = 28; pub const H2:usize = 38; pub const H3:usize = 48; pub const H4:usize = 58;

    pub const A5:usize = 61; pub const A6:usize = 71; pub const A7:usize = 81; pub const A8:usize = 91;
    pub const B5:usize = 62; pub const B6:usize = 72; pub const B7:usize = 82; pub const B8:usize = 92;
    pub const C5:usize = 63; pub const C6:usize = 73; pub const C7:usize = 83; pub const C8:usize = 93;
    pub const D5:usize = 64; pub const D6:usize = 74; pub const D7:usize = 84; pub const D8:usize = 94;
    pub const E5:usize = 65; pub const E6:usize = 75; pub const E7:usize = 85; pub const E8:usize = 95;
    pub const F5:usize = 66; pub const F6:usize = 76; pub const F7:usize = 86; pub const F8:usize = 96;
    pub const G5:usize = 67; pub const G6:usize = 77; pub const G7:usize = 87; pub const G8:usize = 97;
    pub const H5:usize = 68; pub const H6:usize = 78; pub const H7:usize = 88; pub const H8:usize = 98;

    pub const NO_SQ:u8 = 99;
    pub const OFFBOARD:u8 = 100;

    #[inline(always)]
    pub fn is_sq_on_board(sq:i32) -> bool { FILE_SQUARES[sq as usize] != OFFBOARD }
}