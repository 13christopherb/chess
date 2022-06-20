/// Module for creating a unique key for any given board state

use rand::Rng;
use crate::constants::{pieces, squares};

#[derive(Debug, Copy, Clone)]
pub struct BoardHasher {
    pub piece_keys: [[u64; 128]; 13],
    pub side_key: u64,
    pub castle_keys: [u64; 16],
}

impl BoardHasher {
    pub fn new() -> BoardHasher {
        // Need to fill arrays with random numbers for each part of the board state

        let mut rng = rand::thread_rng();
        let mut piece_keys: [[u64; 128]; 13] = [[0; 128]; 13];
        for key in piece_keys.iter_mut().flat_map(|r| r.iter_mut()) {
                *key = rng.gen();
        }
        let side_key: u64 = rng.gen();
        let mut castle_keys: [u64; 16] = [0; 16];
        for key in castle_keys.iter_mut() {
            *key = rng.gen();
        }

        BoardHasher {
            piece_keys,
            side_key,
            castle_keys,
        }
    }

    pub fn generate_key(self, pieces: [u8; 120], side: u8, en_passant: u8, castle_perm: u8) -> u64 {
        let mut sq = 0;
        let mut final_key: u64 = 0;

        for piece in pieces {
            if piece != squares::OFFBOARD && piece != pieces::EMPTY {
                assert!((pieces::WP..=pieces::BK).contains(&piece));
                final_key ^= self.piece_keys[piece as usize][sq];
            }
        }

        // White
        if side == 0 {
            final_key ^= self.side_key;
        }

        if en_passant != squares::NO_SQ as u8 {
            assert!(en_passant < 120);
            final_key ^= self.piece_keys[pieces::EMPTY as usize][usize::try_from(en_passant).unwrap()];
        }

        assert!(castle_perm <= 15);

        final_key ^= self.castle_keys[castle_perm as usize];
        final_key
    }

    #[cfg(test)]
    pub fn seed(piece_keys: [[u64; 128]; 13], side_key: u64, castle_keys: [u64; 16]) -> BoardHasher {
        BoardHasher {
            piece_keys,
            side_key,
            castle_keys,
        }
    }
}


#[cfg(test)]
mod test {
    use crate::utils::hashkeys::BoardHasher;

    #[test]
    fn test_generate_key() {
        let hasher = BoardHasher::new();

        let mut pieces1: [u8; 120] = [0; 120];
        let mut pieces2: [u8; 120] = [0; 120];
        pieces1[45] = 2;
        pieces1[55] = 5;
        pieces2[65] = 4;
        pieces2[34] = 7;
        let side1 = 0;
        let side2 = 1;
        let enpassant1 = 42;
        let enpassant2 = 0;
        let castle1 = 3;
        let castle2 = 5;

        assert_eq!(hasher.generate_key(pieces1, side1, enpassant1, castle1),
                   hasher.generate_key(pieces1, side1, enpassant1, castle1),
                   "Did not produce the same hash for identical board states"
        );

        assert_eq!(hasher.generate_key(pieces2, side2, enpassant2, castle2),
                   hasher.generate_key(pieces2, side2, enpassant2, castle2),
                   "Did not produce the same hash for identical board states"
        );

        assert_ne!(hasher.generate_key(pieces1, side1, enpassant1, castle1),
                   hasher.generate_key(pieces2, side2, enpassant2, castle2),
                   "Did not produce different hashes for different board states"
        );

        assert_ne!(hasher.generate_key(pieces1, side1, enpassant1, castle1),
                   hasher.generate_key(pieces1, side1, enpassant1, castle2),
                   "Did not produce different hashes for different board states"
        );

        assert_ne!(hasher.generate_key(pieces1, side1, enpassant1, castle1),
                   hasher.generate_key(pieces1, side1, enpassant2, castle1),
                   "Did not produce different hashes for different board states"
        );

        assert_ne!(hasher.generate_key(pieces1, side1, enpassant1, castle1),
                   hasher.generate_key(pieces1, side2, enpassant1, castle1),
                   "Did not produce different hashes for different board states"
        );

        assert_ne!(hasher.generate_key(pieces1, side1, enpassant1, castle1),
                   hasher.generate_key(pieces2, side1, enpassant1, castle1),
                   "Did not produce different hashes for different board states"
        );
    }
}