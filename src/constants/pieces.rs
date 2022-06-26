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
pub const PAWN_NUMBER:[bool; 13] = [false, true, false, false, false, false,
    false, true, false, false, false, false, false];
pub const KING_NUMBER:[bool; 13] = [false, false, false, false, false, false,
    true, false, false, false, false, false, true];
pub const SLIDES:[bool; 13] = [false, false, false, true, true, true, false, false, false, true, true, true, false];

pub const BIG_PIECE:[bool; 13] = [ false, false, true, true, true, true, true, false, true, true, true, true, true ];
pub const MAJOR_PIECE:[bool; 13] = [ false, false, false, false, true, true, true, false, false, false, true, true, true ];
pub const MINOR_PIECE:[bool; 13] = [ false, false, true, true, false, false, false, false, true, true, false, false, false ];
pub const LOOP_SLIDE:[u8; 8] = [WB, WR, WQ, 0, BB, BR, BQ, 0];
pub const LOOP_SLIDE_INDEX:[usize; 2] = [0, 4];
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
pub const WK_CASTLE:u8 = 1;
pub const WQ_CASTLE:u8 = 2;
pub const BK_CASTLE:u8 = 3;
pub const BQ_CASTLE:u8 = 4;




