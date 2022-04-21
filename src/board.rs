use std::collections::HashMap;
use std::fmt;
use regex::Regex;

use crate::piece::{Piece, Pawn, Color, Blank};

#[derive(Debug)]
pub struct Square {
    pub file: char,
    pub rank: char,
    pub piece: Piece,
}

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<Square>>
}


impl Board {
    fn get_possible_moves(&self, piece: Piece) {
        //piece.possible_moves(self.board);
    }

    pub fn parse_move(&self, move_string: &str) -> Result<((char, char), (char, char)), &str> {
        let re = Regex::new("[a-h][1-8] [a-h][1-8]").unwrap();
        match re.is_match(move_string) {
            true => {
                let coordinates: Vec<&str> = move_string.trim().split(" ").collect();
                let src_coordinates: Vec<char> = coordinates[0].chars().collect();
                let dest_coordinates: Vec<char> = coordinates[1].chars().collect();
                Ok(((src_coordinates[0], src_coordinates[1]), (dest_coordinates[0], dest_coordinates[1])))
            },
            false => Err("Incorrect input!"),
        }
    }

    pub fn move_piece(&mut self, src: (char, char), dest: (char, char)) {
        let src_coord = &self.convert_coordinate(src.0, src.1);
        let dest_coord = &self.convert_coordinate(dest.0, dest.1);
        let piece = &self.board[src_coord.0][src_coord.1].piece;
        let mut old = std::mem::replace(&mut self.board[src_coord.0][src_coord.1], Square { file: src.0, rank: src.1, piece: Piece::Blank });
        old.file = dest.0;
        old.rank = dest.1;
        let new = std::mem::replace(&mut self.board[dest_coord.0][dest_coord.1], old);
    }

    fn convert_coordinate(&self, file: char, rank: char) -> (usize, usize) {
        let files: HashMap<char, usize> = HashMap::from([
            ('a', 0),
            ('b', 1),
            ('c', 2),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('h', 7),
        ]);
        let ranks: HashMap<char, usize> = HashMap::from([
            ('8', 0),
            ('7', 1),
            ('6', 2),
            ('5', 3),
            ('4', 4),
            ('3', 5),
            ('2', 6),
            ('1', 7),
        ]);
        return (*ranks.get(&rank).unwrap(), *files.get(&file).unwrap());
    }

    pub fn from_fen(fen: String) -> Board {
        let fen_split: Vec<&str> = fen.split(" ").collect();
            
        let board_layout = fen_split[0].split("/");

        let mut board: Vec<Vec<Square>> = Vec::with_capacity(8 * 8);

        let get_new_piece = |piece: char| {
            match piece {
                'P' => Piece::Pawn,
                'B' => Piece::Bishop,
                'Q' => Piece::Queen,
                'K' => Piece::King,
                'N' => Piece::Knight,
                'R' => Piece::Rook,
                _   => Piece::Blank
            }
        };

        for (i, row) in board_layout.enumerate() {
            board.push(Vec::new());
            for (j, piece) in row.chars().enumerate() {
                board[i].push(Square {
                    rank: '1',
                    file: 'a',
                    piece: get_new_piece(piece)
                });
            }
        }


        Self { board }
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
        Piece::Knight => 'N',
        Piece::Queen => 'Q',
        Piece::King => 'K',
        Piece::Bishop => 'B',
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
