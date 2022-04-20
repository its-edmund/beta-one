use crate::board::Board;

type Coordinate = (u8, u8);

#[derive(Debug)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Pawn,
    Rook,
    Knight,
    Blank
}

struct Player {
    color: Color
}

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

pub struct Pawn {
    has_moved: bool,
    location: Coordinate,
    killed: bool,
    color: Color,
    piece_id: char
}

pub struct Rook;

pub struct Blank;

fn get_piece_id(piece: Piece) -> char {
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

/*
 *fn get_legal_moves(piece: Piece, board: Board) -> Vec<Coordinate> {
 *    match piece {
 *        Piece::Pawn => pawn_legal_moves(),
 *        _ => pawn_legal_moves(board)
 *    }
 *}
 *
 *fn pawn_legal_moves(board: Board, file: char, rank: char) -> Vec<Coordinate> {
 *
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


impl Rook { 
    fn possible_moves(&self, board: Board) -> Vec<Coordinate> {
        let mut moves: Vec<Coordinate> = Vec::new();
        moves
    }
}
