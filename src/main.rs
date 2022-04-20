use board::Board;
use board::Square;
use piece::Piece;
use piece::Blank;

pub mod piece;
pub mod board;

fn main() {
    let mut board_layout: Vec<Vec<Square>> = Vec::with_capacity(8 * 8);
    for i in 0..8 {
        board_layout.push(Vec::new());
        for j in 0..8 {
            board_layout[0].push(Square {
                rank: '1',
                file: 'a',
                piece: Piece::Blank
            });
        }
    }

    let board = Board { board: board_layout };
}
