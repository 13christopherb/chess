pub mod hash_keys {
    use rand::Rng;
    use crate::Board;
    use crate::board::Pieces;
    use crate::board::Squares;
    use crate::board::Files;


    pub struct BoardHasher {
        piece_keys: [[u64; 13]; 120],
        side_key: u64,
        castle_keys: [u64; 16],
    }

    impl BoardHasher {
        pub fn new() -> BoardHasher {
            let mut rng = rand::thread_rng();
            let mut piece_keys: [[u64; 13]; 120] = [[0; 13]; 120];
            for i in 0..13 {
                for j in 0..120 {
                    piece_keys[i][j] = rng.gen();
                }
            }
            let side_key: u64 = rng.gen();
            let mut castle_keys: [u64; 16] = [0; 16];
            for i in 0..16 {
                castle_keys[i] = rng.gen();
            }

            BoardHasher {
                piece_keys,
                side_key,
                castle_keys,
            }
        }

        pub fn generate_key(self, pieces:[u8; 120], side:u8, en_passant:u64, castle_perm:u8) -> u64 {
            let mut sq = 0;
            let mut final_key:u64 = 0;

            for piece in pieces {
                if piece != Squares::NoSq as u8 && piece != Pieces::EMPTY as u8 {
                    assert!(piece >= Pieces::WP as u8 && piece <= Pieces::BK as u8);
                    final_key ^= self.piece_keys[piece as usize][sq];
                }
            }

            // White
            if side == 0 {
                final_key ^= self.side_key;
            }

            if en_passant != Squares::NoSq as u64 {
                assert!(en_passant >= 0 && en_passant < 120);
                final_key ^= self.piece_keys[Pieces::EMPTY as usize][usize::try_from(en_passant).unwrap()];
            }

            assert!(castle_perm <= 15);

            final_key ^= self.castle_keys[castle_perm as usize];
            final_key

        }

        #[cfg(test)]
        pub fn seed(piece_keys: [[u64; 13]; 120], side_key: u64, castle_keys: [u64; 16]) -> BoardHasher {
            BoardHasher {
                piece_keys,
                side_key,
                castle_keys,
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::hashkeys::hash_keys::BoardHasher;
//
//     #[test]
//     fn test_fr2sq() {
//         let mut piece_keys:[[u64; 13]; 120] = [[0; 13]; 120];
//         let side_key: u64 = 1;
//         let mut castle_keys: [u64; 16] = [0; 16];
//
//         for i in 0..13 {
//             for j in 0..120 {
//                 piece_keys[i][j] = i as u64 * 13 + j as u64;
//             }
//         }
//
//         for i in 0..16 {
//             castle_keys[i] = i as u64;
//         }
//
//         let test = BoardHasher::seed(piece_keys, side_key, castle_keys);
//         assert_eq!(square,
//                    74,
//                    "Did not convert file and rank into correct square"
//         );
//     }
//
// }