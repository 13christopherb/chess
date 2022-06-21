use crate::constants::pieces::{*};
use crate::constants::squares::{*};

#[inline(always)]
fn is_side_valid(side:u8) -> bool { side == WHITE || side == BLACK }

#[inline(always)]
fn is_filerank_valid(fr: u8) -> bool {
    // fr needs to be >= 0
    fr <= 7
}

#[inline(always)]
fn is_piece_valid(pce: u8) -> bool { pce <= BK }

#[inline(always)]
fn is_piece(pce: u8) -> bool { (WP..=BK).contains(&pce) }

#[inline(always)]
pub fn is_sq_on_board(sq:u8) -> bool { FILE_SQUARES[sq as usize] != OFFBOARD }

#[inline(always)]
pub fn is_knight(pce:u8) -> bool { KNIGHT_NUMBER[pce as usize] }

#[inline(always)]
pub fn is_queen(pce:u8) -> bool { pce == WQ || pce == BQ }


#[inline(always)]
pub fn piece_slides(pce:u8) -> bool { SLIDES[pce as usize] }

#[inline(always)]
pub fn is_king(pce:u8) -> bool { KING_NUMBER[pce as usize] }

#[inline(always)]
pub fn is_same_color(pce:u8, color:u8) -> bool { PIECE_COLOR[pce as usize] == color }

#[inline(always)]
pub fn opposite_color(pce:u8, color:u8) -> bool { PIECE_COLOR[pce as usize] == color ^ 1 }

#[inline(always)]
pub fn is_white(pce:u8,) -> bool {pce != OFFBOARD && PIECE_COLOR[pce as usize] == WHITE }

#[inline(always)]
pub fn is_black(pce:u8,) -> bool {pce != OFFBOARD && PIECE_COLOR[pce as usize] == BLACK }
