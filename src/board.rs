use regex::Regex;
use std::collections::HashMap;
use std::fmt;

use crate::piece::{Blank, Color, Pawn, Piece};

#[derive(Debug, Clone)]
pub struct Square {
    pub file: char,
    pub rank: char,
    pub piece: Piece,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub board: Vec<Vec<Square>>,
    pub current_move: Color,
    pub black_can_castle_ks: bool,
    pub black_can_castle_qs: bool,
    pub white_can_castle_ks: bool,
    pub white_can_castle_qs: bool,
    pub en_passant_target: String,
    pub halfmove_clock: i32,
    pub fullmove_number: i32,
}

impl Board {
    fn get_possible_moves(&self, piece: Piece) {
        //piece.possible_moves(self.board);
    }

    pub fn parse_move(move_string: &str) -> Result<((char, char), (char, char)), &str> {
        let re = Regex::new("[a-h][1-8] [a-h][1-8]").unwrap();
        match re.is_match(move_string) {
            true => {
                let coordinates: Vec<&str> = move_string.trim().split(" ").collect();
                let src_coordinates: Vec<char> = coordinates[0].chars().collect();
                let dest_coordinates: Vec<char> = coordinates[1].chars().collect();
                Ok((
                    (src_coordinates[0], src_coordinates[1]),
                    (dest_coordinates[0], dest_coordinates[1]),
                ))
            }
            false => Err("Incorrect input!"),
        }
    }

    pub fn check_color_piece(&mut self, src: (char, char)) -> bool {
        let src_coord = &self.convert_coordinate(src.0, src.1);
        let piece = &self.board[src_coord.0][src_coord.1].piece;
        match piece {
            Piece::Blank => false,
            Piece::King { color }
            | Piece::Queen { color }
            | Piece::Rook { color }
            | Piece::Bishop { color }
            | Piece::Pawn { color }
            | Piece::Knight { color } => {
                std::mem::discriminant(color) == std::mem::discriminant(&self.current_move)
            }
        }
    }

    pub fn move_piece(&mut self, src: (char, char), dest: (char, char)) {
        let src_coord = &self.convert_coordinate(src.0, src.1);
        let dest_coord = &self.convert_coordinate(dest.0, dest.1);
        let piece: &Piece = &self.board[src_coord.0][src_coord.1].piece;
        // Check for legal moves
        let _is_legal = match piece {
            Piece::Blank => unimplemented!(),
            Piece::King { .. }  => unimplemented!(),
            Piece::Queen { .. } => unimplemented!(),
            Piece::Bishop { .. } => unimplemented!(),
            Piece::Pawn { .. } => unimplemented!(),
            Piece::Rook { .. } => unimplemented!(),
            Piece::Knight { .. } => unimplemented!(),
        };
        // let mut old = std::mem::replace(
        //     &mut self.board[src_coord.0][src_coord.1],
        //     Square {
        //         file: src.0,
        //         rank: src.1,
        //         piece: Piece::Blank,
        //     },
        // );
        // old.file = dest.0;
        // old.rank = dest.1;
        // let new = std::mem::replace(&mut self.board[dest_coord.0][dest_coord.1], old);
    }

    pub fn toggle_current_move(&mut self) {
        self.current_move = if matches!(self.current_move, Color::BLACK) {
            Color::WHITE
        } else {
            Color::BLACK
        }
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

        let current_move = fen_split[1];

        let can_castle = fen_split[2];

        let en_passant_target = fen_split[3];

        let halfmove_clock = fen_split[4];

        let fullmove_number = fen_split[5];

        let mut board: Vec<Vec<Square>> = Vec::with_capacity(8 * 8);

        let get_new_piece = |piece: char| match piece {
            'P' => Piece::Pawn {
                color: Color::WHITE,
            },
            'p' => Piece::Pawn {
                color: Color::BLACK,
            },
            'B' => Piece::Bishop {
                color: Color::WHITE,
            },
            'b' => Piece::Bishop {
                color: Color::BLACK,
            },
            'Q' => Piece::Queen {
                color: Color::WHITE,
            },
            'q' => Piece::Queen {
                color: Color::BLACK,
            },
            'K' => Piece::King {
                color: Color::WHITE,
            },
            'k' => Piece::King {
                color: Color::BLACK,
            },
            'N' => Piece::Knight {
                color: Color::WHITE,
            },
            'n' => Piece::Knight {
                color: Color::BLACK,
            },
            'R' => Piece::Rook {
                color: Color::WHITE,
            },
            'r' => Piece::Rook {
                color: Color::BLACK,
            },
            _ => Piece::Blank,
        };

        for (i, row) in board_layout.enumerate() {
            board.push(Vec::new());
            for (j, piece) in row.chars().enumerate() {
                if piece.is_numeric() {
                    for k in 0..piece.to_digit(10).unwrap() {
                        board[i].push(Square {
                            rank: '1',
                            file: 'a',
                            piece: Piece::Blank,
                        });
                    }
                } else {
                    board[i].push(Square {
                        rank: '1',
                        file: 'a',
                        piece: get_new_piece(piece),
                    });
                }
            }
        }

        Self {
            board,
            black_can_castle_ks: can_castle.contains('k'),
            black_can_castle_qs: can_castle.contains('q'),
            white_can_castle_ks: can_castle.contains('K'),
            white_can_castle_qs: can_castle.contains('Q'),
            en_passant_target: String::from(en_passant_target),
            current_move: if current_move.eq("w") {
                Color::WHITE
            } else {
                Color::BLACK
            },
            halfmove_clock: halfmove_clock.parse::<i32>().unwrap(),
            fullmove_number: fullmove_number.parse::<i32>().unwrap(),
        }
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
        Piece::Pawn {
            color: Color::WHITE,
        } => 'P',
        Piece::Pawn {
            color: Color::BLACK,
        } => 'p',
        Piece::Rook {
            color: Color::WHITE,
        } => 'R',
        Piece::Rook {
            color: Color::BLACK,
        } => 'r',
        Piece::Knight {
            color: Color::WHITE,
        } => 'N',
        Piece::Knight {
            color: Color::BLACK,
        } => 'n',
        Piece::Queen {
            color: Color::WHITE,
        } => 'Q',
        Piece::Queen {
            color: Color::BLACK,
        } => 'q',
        Piece::King {
            color: Color::WHITE,
        } => 'K',
        Piece::King {
            color: Color::BLACK,
        } => 'k',
        Piece::Bishop {
            color: Color::WHITE,
        } => 'B',
        Piece::Bishop {
            color: Color::BLACK,
        } => 'b',
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
        display_string.push_str(&format!("It\'s {}\'s move right now!\n", self.current_move));
        write!(f, "{}", display_string)
    }
}
