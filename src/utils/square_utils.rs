use crate::constants::{squares};

pub fn fr2sq(file: u8, rank: u8) -> u8 {
    (21 + file) + (rank * 10)
}

/// Initializes arrays that have either the file or rank number for each square (or offboard
/// if the 120 board square is off the 8x8 board
pub fn init_file_rank_arrays() -> ([u8; 120], [u8; 120]) {
    let mut files: [u8; 120] = [0; 120];
    let mut ranks: [u8; 120] = [0; 120];

    for i in 0..120 {
        files[i] = squares::OFFBOARD;
        ranks[i] = squares::OFFBOARD;
    }

    for rank in squares::RANK_1..=squares::RANK_8 {
        for file in squares::FILE_A..=squares::FILE_H {
            let sq = fr2sq(file, rank) as usize;
            files[sq] = file;
            ranks[sq] = rank;
        }
    }
    (files, ranks)
}