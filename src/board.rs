use std::collections::HashMap;
use std::fmt;

use crate::piece::{Piece, Pawn, Color};

pub struct Square {
    pub rank: char,
    pub file: char,
    pub piece: Piece,
}

pub struct Board {
    pub board: Vec<Vec<Square>>
}

impl Board {
    fn show_state(&self) {
        for i in 0..8 {
            for j in 0..8{
                //let square = self.board[i][i];
                /*
                 *let piece = square.piece.get_piece_id();
                 *print!("{} ", square.piece);
                 */
            }
            println!("");
        }
    }
    
    fn get_possible_moves(&self, piece: Piece) {
        //piece.possible_moves(self.board);
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

    fn new() {
    
    }

    fn from_fen(&self, fen: String) {

    }

    fn initialize_board(&self) {
/*
 *        for i in 0..8 {
 *            for j in 0..8 {
 *                if i == 1 {
 *
 *                    self.board[i][j] = Square { rank: '1', file: 'a', piece: Box::new(Pawn {
 *                            has_moved: false,
 *                            location: (i as u8, j as u8),
 *                            killed: false,
 *                            color: Color::BLACK,
 *                            piece_id: 'P',
 *                        }),
 *                    }
 *                }
 *                let square = &self.board[i][i];
 *                print!("{} ", square.piece.piece_id);
 *            }
 *            println!("");
 *        }
 */
    }
}

fn get_piece_id(piece: &Piece) -> char {
    match piece {
        Piece::Pawn => 'P',
        Piece::Rook => 'R',
        Piece::Blank => '_',
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display_string = String::new();
        for rank in &self.board {
            for square in rank {
                display_string.push(get_piece_id(&square.piece));
                display_string.push(' ');
            }
            display_string.push('\n');
        } 
        write!(f, "{}", display_string)
    }
}
