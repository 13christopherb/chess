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

pub const A1:usize = 21;
pub const A2:usize = 31;
pub const A3:usize = 41;
pub const A4:usize = 51;
pub const B1:usize = 22;
pub const B2:usize = 32;
pub const B3:usize = 42;
pub const B4:usize = 52;
pub const C1:usize = 23;
pub const C2:usize = 33;
pub const C3:usize = 43;
pub const C4:usize = 53;
pub const D1:usize = 24;
pub const D2:usize = 34;
pub const D3:usize = 44;
pub const D4:usize = 54;
pub const E1:usize = 25;
pub const E2:usize = 35;
pub const E3:usize = 45;
pub const E4:usize = 55;
pub const F1:usize = 26;
pub const F2:usize = 36;
pub const F3:usize = 46;
pub const F4:usize = 56;
pub const G1:usize = 27;
pub const G2:usize = 37;
pub const G3:usize = 47;
pub const G4:usize = 57;
pub const H1:usize = 28;
pub const H2:usize = 38;
pub const H3:usize = 48;
pub const H4:usize = 58;

pub const A5:usize = 61;
pub const A6:usize = 71;
pub const A7:usize = 81;
pub const A8:usize = 91;
pub const B5:usize = 62;
pub const B6:usize = 72;
pub const B7:usize = 82;
pub const B8:usize = 92;
pub const C5:usize = 63;
pub const C6:usize = 73;
pub const C7:usize = 83;
pub const C8:usize = 93;
pub const D5:usize = 64;
pub const D6:usize = 74;
pub const D7:usize = 84;
pub const D8:usize = 94;
pub const E5:usize = 65;
pub const E6:usize = 75;
pub const E7:usize = 85;
pub const E8:usize = 95;
pub const F5:usize = 66;
pub const F6:usize = 76;
pub const F7:usize = 86;
pub const F8:usize = 96;
pub const G5:usize = 67;
pub const G6:usize = 77;
pub const G7:usize = 87;
pub const G8:usize = 97;
pub const H5:usize = 68;
pub const H6:usize = 78;
pub const H7:usize = 88;
pub const H8:usize = 98;

pub const NO_SQ:u8 = 99;
pub const OFFBOARD:u8 = 100;
