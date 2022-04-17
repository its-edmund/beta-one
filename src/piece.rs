use crate::board::Board;

type Coordinate = (i16, i16);

struct Player {
    color: Color
}

enum Color {
    BLACK,
    WHITE
}

pub struct Piece {
    location: Coordinate,
    killed: bool,
    color: Color,
    player: Player
}

pub trait Movable {
    fn possible_moves(&self) -> Vec<Coordinate>;
}

pub struct Pawn {
    piece: Piece,
    has_moved: bool
}

impl Pawn {
    fn possible_moves(&self, board: Board) -> Vec<Coordinate> {
        let mut moves: Vec<Coordinate> = Vec::new();
        if !self.has_moved {
            moves.push((self.piece.location.0 - 1, self.piece.location.1));
            moves.push((self.piece.location.0 - 2, self.piece.location.1));
        } else {
            moves.push((self.piece.location.0 - 1, self.piece.location.1));
        }

        return moves;
    }
}


