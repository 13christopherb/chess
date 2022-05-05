use crate::constants::pieces;

#[inline(always)]
fn is_side_valid(side:u8) -> bool { side == pieces::WHITE || side == pieces::BLACK }

#[inline(always)]
fn is_filerank_valid(fr: u8) -> bool { fr >= 0 && fr <= 7 }

#[inline(always)]
fn is_piece_valid(pce: u8) -> bool { pce >= pieces::EMPTY && pce <= pieces::BK }

#[inline(always)]
fn is_piece(pce: u8) -> bool { pce >= pieces::WP && pce <= pieces::BK }