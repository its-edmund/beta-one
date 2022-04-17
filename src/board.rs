use crate::piece::Piece;

struct Square {
    rank: String,
    file: String,
    piece: Piece,
    piece_id: String,
}

pub struct Board {
    board: Vec<Vec<Square>>
}

impl Board {
    fn show_state(&self) {
        for x in 0..7 {
            for y in 0..7 {
                /*
                 *print!(board[x][y]);
                 */
            }
        }
    }

    fn move_piece(&self, row: usize, col: usize) {
        let piece = &self.board[col][row];
    }

    fn convertCoordinate(&self, rank: char, file: char) -> (u8, u8) {

    }
}
