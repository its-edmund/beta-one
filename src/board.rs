use std::collections::HashMap;

use crate::piece::{Piece, Pawn, Color};

struct Square<T: Piece> {
    rank: char,
    file: char,
    piece: Box<T>,
}

pub struct Board<T> {
    board: Vec<Vec<Square<T>>>
}

impl<T> Board<T> {
    fn show_state(&self) {
        for i in 0..8 {
            for j in 0..8{
                let square = self.board[i][i];
                let piece = *square.piece;
                print!("{} ", square.piece.piece_id);
            }
            println!("");
        }
    }
    
    fn get_possible_moves(&self, piece: T) {
        piece.possible_moves(self.board);
    }

    fn move_piece(&self, row: usize, col: usize) {
        let piece = &self.board[col][row];
    }

    fn convert_coordinate(&self, rank: &char, file: &char) -> (u8, u8) {
        let files: HashMap<char, u8> = HashMap::from([
            ('a', 0),
            ('b', 1),
            ('c', 2),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('h', 7),
        ]);
        let ranks: HashMap<char, u8> = HashMap::from([
            ('8', 0),
            ('7', 1),
            ('6', 2),
            ('5', 3),
            ('4', 4),
            ('3', 5),
            ('2', 6),
            ('1', 7),
        ]);
        return (*ranks.get(rank).unwrap(), *files.get(file).unwrap());
    }

    fn initialize_board(&self) {
        for i in 0..8 {
            for j in 0..8 {
                if i == 1 {

                    self.board[i][j] = Square { rank: '1', file: 'a', piece: Box::new(Pawn {
                            has_moved: false,
                            location: (i as u8, j as u8),
                            killed: false,
                            color: Color::BLACK,
                            piece_id: 'P',
                        }),
                    }
                }
                let square = &self.board[i][i];
                print!("{} ", square.piece.piece_id);
            }
            println!("");
        }
    }
}
