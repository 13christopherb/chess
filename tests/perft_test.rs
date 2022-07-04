use chess::game_board::board::Board;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
use chess::constants::squares::{FILE_E, FILE_F, RANK_8};
use chess::moves::gamemove::GameMove;
use chess::moves::movegen::generate_all_moves;
use chess::utils::square_utils::fr2sq;

#[derive(Clone)]
struct PositionCounts {
    pub fen:String,
    pub nums: Vec<u64>
}

impl FromStr for PositionCounts {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fen, num_positions) = match s.split_once(";") {
            Some(a) => a,
            None => panic!("Parsing error")
        };

        let nums = num_positions.split_whitespace().filter_map(|x| x.parse::<u64>().ok()).collect();

        Ok(PositionCounts{
            fen: fen.to_string(),
            nums
        })
    }
}

fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

fn perft(depth: u8, board: &mut Board, movs: & Vec<GameMove>) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut move_number = 0;

    let mut movelist: Vec<GameMove> = Vec::new();
    generate_all_moves(board, &mut movelist);

    for mov in movelist {
      if !board.make_move(mov) { continue; }

        let mut mov_list = movs.clone();
        mov_list.push(mov);


        move_number += perft(depth - 1, board, &mov_list);

        for m in mov_list {
            print!("{}=> ", m);
        }
        print!("\n");
        board.undo_move();
    }
    move_number
}

#[test]
#[ignore]
fn perft_test() {
    let positions: Vec<PositionCounts> = read_all::<PositionCounts>("tests/perftsuite.txt")
        .iter()
        .filter_map(|x| x.clone().ok())
        .collect();

    for i in 2..positions.len() {
        let mut board = Board::new();
        unsafe{ board.parse_fen(positions[i].fen.as_str()) };
        board.update_material_list();
        for j in 0..4 {
            let move_number = perft((j + 1) as u8, &mut board, &Vec::new());
            assert_eq!(move_number, positions[i].nums[j], "Did not find ocrrect number of moves for position {} at depth {}", i, j);
        }
    }
}
