pub const MULTIPLICATOR:u64 = 0x03f79d71b4cb0a89;

static DEBRUIJ_TABLE: &[u8] = &[
    0, 47,  1, 56, 48, 27,  2, 60,
    57, 49, 41, 37, 28, 16,  3, 61,
    54, 58, 35, 52, 50, 42, 21, 44,
    38, 32, 29, 23, 17, 11,  4, 62,
    46, 55, 26, 59, 40, 36, 15, 53,
    34, 51, 20, 43, 31, 22, 10, 45,
    25, 39, 14, 33, 19, 30,  9, 24,
    13, 18,  8, 12,  7,  6,  5, 63
];

/// Locates the index of the least significant bit
///
/// # Panic
/// Will panic if parameter is empty
fn bitscan_forward(x:u64) -> u8 {
    let mut lsb = x ^ x - 1;
    lsb = lsb.wrapping_mul(MULTIPLICATOR);
    lsb >>= 58;
    DEBRUIJ_TABLE[lsb as usize]
}

#[cfg(test)]
mod test {
    use crate::bit_operations::bitscan_forward;

    #[test]
    fn test_bitscan_forward() {
        let index = bitscan_forward(0x1000);
        assert_eq!(index,
                   12,
                   "Did not correctly find index of least significant bit"
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_bitboard() {
        let index = bitscan_forward(0);
    }

}