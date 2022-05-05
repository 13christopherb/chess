/*
    This uses an integer to represent all the data in a move with
    the following bits representing each part:
    0000 0000 0000 0000 0000 0111 1111 -> Origin square 0x7F
    0000 0000 0000 0011 1111 1000 0000 -> To square >> 7 0x3F
    0000 0000 0011 1100 0000 0000 0000 -> Captured >> 14 0xF
    0000 0000 0100 0000 0000 0000 0000 -> En passant 0x40000
    0000 0000 1000 0000 0000 0000 0000 -> Pawn start 0x80000
    0000 1111 0000 0000 0000 0000 0000 -> Promoted piece >> 20 0xF
    0001 0000 0000 0000 0000 0000 0000 -> Castle 0x1000000
 */

pub const MFLAG_EP:u32 = 0x40000; // En passant
pub const MFLAG_PS:u32 = 0x80000; // Pawn start
pub const MFLAG_CA:u32 = 0x1000000; // Castle
pub const MFLAG_CAP:u32 = 0x7C000; // Capture
pub const MFLAG_PROM:u32 = 0xF00000; // Promotion

#[derive(Debug, Copy, Clone)]
pub struct GameMove {
    pub move_int: u32,
    pub score: u8,
}

impl GameMove {

    #[inline(always)]
    pub fn new(from:u8, to:u8, cap:u8,
               prom:u8, flag:u32) -> GameMove {
        let move_int:u32 = (from as u32) | (to as u32) << 7 | (cap as u32) << 14 | (prom as u32) << 20 | flag;
        GameMove {
            move_int,
            score: 0
        }
    }
    #[inline(always)]
    pub fn origin(self) -> u8 { (self.move_int & 0x7F) as u8 }

    #[inline(always)]
    pub fn destination(self) -> u8 { (self.move_int >> 7 & 0x7F) as u8 }

    #[inline(always)]
    pub fn capture(self) -> u8 { (self.move_int >> 14 & 0xF) as u8 }

    #[inline(always)]
    pub fn promoted_piece(self) -> u8 { (self.move_int >> 20 & 0xF) as u8 }

    #[inline(always)]
    pub fn pawn_start(&self) -> bool { (self.move_int & MFLAG_PS) > 0}
}