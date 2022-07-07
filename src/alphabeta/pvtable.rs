use std::mem;
use std::mem::MaybeUninit;
use crate::game_board::board::Board;
use crate::moves::gamemove::GameMove;

#[derive(Debug, Clone)]
pub struct PVEntry {
    poskey: u64,
    mov: GameMove,
}

#[derive(Debug, Clone)]
pub struct PVTable {
    pub ptable: Vec<PVEntry>,
}

impl PVTable {
    pub(crate) fn new() -> PVTable {
        let mut table = vec![PVEntry{
            poskey: 0,
            mov: GameMove { move_int: 0, score: 0 }
        }; 100000];
        PVTable { ptable: table }
    }

    #[inline]
    pub fn store(&mut self, pos: &Board, mov:GameMove) {
        let index = pos.pos_key as usize % self.ptable.len();

        self.ptable[index].poskey = pos.pos_key;
        self.ptable[index].mov = mov;
    }

    #[inline]
    pub fn probe(&self, pos: &Board) -> Option<GameMove> {
        let index = pos.pos_key as usize % self.ptable.len();

        if self.ptable[index].poskey == pos.pos_key {
            return Some(self.ptable[index].mov);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::alphabeta::pvtable::PVTable;
    use crate::constants::squares::{*};
    use crate::game_board::board::Board;
    use crate::moves::gamemove::GameMove;
    use crate::utils::square_utils::fr2sq;

    #[test]
    fn test_store() {
        let mut board = Board::new();
        unsafe{board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")};
        board.update_material_list();

        let mut table = PVTable::new();
        let mov = GameMove::new(fr2sq(FILE_E, RANK_2), fr2sq(FILE_E, RANK_3), 0, 0, 0);
        board.pos_key = 200000;
        table.store(&board, mov);
        assert_eq!(table.ptable[200000 % table.ptable.len()].mov.move_int, mov.move_int);
    }

    #[test]
    fn test_probe() {
        let mut board = Board::new();
        unsafe{board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")};
        board.update_material_list();

        let mut table = PVTable::new();
        let length = table.ptable.len();
        let mov = GameMove::new(fr2sq(FILE_E, RANK_2), fr2sq(FILE_E, RANK_3), 0, 0, 0);
        board.pos_key = 200000;
        table.ptable[200000 % length].mov = mov;
        table.ptable[200000 % length].poskey = board.pos_key;
        let retrieved_move = table.probe(&board).unwrap();
        assert_eq!(retrieved_move.move_int, mov.move_int);
    }
}