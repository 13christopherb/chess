use std::mem;
use std::mem::MaybeUninit;
use crate::moves::gamemove::GameMove;

const PV_SIZE:usize = 0x100000 * 2;
const TABLE_SIZE: usize = PV_SIZE / mem::size_of::<PVEntry>() - 2;

#[derive(Debug, Clone)]
pub struct PVEntry {
    poskey: u64,
    mov: GameMove,
}

#[derive(Debug, Clone)]
pub struct PVTable {
    ptable: Vec<PVEntry>,
}

impl PVTable {
    pub(crate) fn new() -> PVTable {
        let mut table = vec![PVEntry{
            poskey: 0,
            mov: GameMove { move_int: 0, score: 0 }
        }; TABLE_SIZE];
        PVTable { ptable: table }
    }

    fn clear_table(&mut self) {

    }
}