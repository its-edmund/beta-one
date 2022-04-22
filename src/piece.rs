use crate::board::Board;
use std::fmt;

type Coordinate = (u8, u8);

#[derive(Debug)]
pub enum Piece {
    King { color: Color },
    Queen { color: Color },
    Bishop { color: Color },
    Pawn { color: Color },
    Rook { color: Color },
    Knight { color: Color },
    Blank
}

struct Player {
    color: Color
}

#[derive(Debug)]
pub enum Color {
    BLACK,
    WHITE
}

/*
 *pub struct Piece<T: Piece> {
 *    location: Coordinate,
 *    killed: bool,
 *    color: Color,
 *    player: Player,
 *    pub piece_id: char
 *}
 */

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



pub struct Pawn {
    has_moved: bool,
    location: Coordinate,
    killed: bool,
    color: Color,
    piece_id: char
}

pub struct Rook;

pub struct Blank;

/*
 *fn get_legal_moves(piece: Piece, board: Board, file: char, rank: char) -> Vec<Coordinate> {
 *    match piece {
 *        Piece::Pawn => pawn_legal_moves(),
 *        _ => pawn_legal_moves(board)
 *    }
 *}
 */

/*
 *fn pawn_legal_moves(board: Board, file: char, rank: char) -> Vec<Coordinate> {
 *    let mut moves: Vec<Coordinate> = Vec::new();
 *    let location = Board::convert_coordinate(rank, file);
 *    if has_moved {
 *        moves.push((location.0 - 1, location.1));
 *        moves.push((location.0 - 2, location.1));
 *        if board.board[location.0 - 1][location.1 + 1] != '_' {
 *            moves.push((location.0 - 1, location.1 + 1))
 *        }
 *        if board.board[location.0 - 1][location.1 - 1] != '_' {
 *            moves.push((location.0 - 1, location.1 - 1))
 *        }
 *    } else {
 *        moves.push((location.0 - 1, location.1));
 *    }
 *    moves
 *}
 */

/*
 *fn rook_legal_moves(board: Board, file: char, rank: char) -> Vec<Coordinate> {
 *    let mut moves: Vec<Coordinate> = Vec::new();
 *    let location = Board::convert_coordinate(rank, file);
 *    while 
 *}
 */


impl Blank {
    fn possible_moves(&self, board: Board) {

    }
}

impl Pawn {
    fn possible_moves(&self, board: Board) -> Vec<Coordinate> {
        let mut moves: Vec<Coordinate> = Vec::new();
        if !self.has_moved {
            moves.push((self.location.0 - 1, self.location.1));
            moves.push((self.location.0 - 2, self.location.1));
            /* if board.board[self.location.0 - 1][self.location.1] != '_' {
                moves.push((self.location.0 - 1, self.location.1))
            } */
        } else {
            /*
             *moves.push((self.piece.location.0 - 1, self.piece.location.1));
             */
        }
        moves
    }

    fn get_piece_id(&self) {
        self.piece_id;
    }
}

