use crate::board::Board;

type Coordinate = (u8, u8);

struct Player {
    color: Color
}

pub enum Color {
    BLACK,
    WHITE
}

pub trait Piece {
    fn possible_moves(&self, board: Board) -> Vec<Coordinate>;

    fn get_piece_id(&self) -> char;
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

pub struct Pawn {
    has_moved: bool,
    location: Coordinate,
    killed: bool,
    color: Color,
    piece_id: char
}

pub struct Rook;

impl Piece for Pawn {
    fn possible_moves(&self, board: Board) -> Vec<Coordinate> {
        let mut moves: Vec<Coordinate> = Vec::new();
        if !self.has_moved {
            moves.push((self.location.0 - 1, self.location.1));
            moves.push((self.location.0 - 2, self.location.1));
            if board.board[self.location.0 - 1][self.location.1] != '0' {
                moves.push((self.location.0 - 1, self.location.1))
            }
        } else {
            moves.push((self.piece.location.0 - 1, self.piece.location.1));
        }
        moves
    }

    fn get_piece_id(&self) {
        self.piece_id;
    }
}


impl Piece for Rook { 
    fn possible_moves(&self, board: Board) -> Vec<Coordinate> {
        let mut moves: Vec<Coordinate> = Vec::new();
        moves
    }
    
    fn get_piece_id(&self) {
        self.piece_id;
    }
}
