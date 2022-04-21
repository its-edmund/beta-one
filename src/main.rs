use board::Board;
use board::Square;
use piece::Piece;
use piece::Blank;
use std::io::{self, Write};

pub mod piece;
pub mod board;

fn main() {
    let mut board = Board::from_fen(String::from("rnbqkbnr/pppppppp/********/********/********/********/PPPPPPPP/RNBQKBNR"));
    print!("{}", board);

    let mut move_input = String::new();
    loop {
        move_input.clear();
        io::stdout().flush();
        std::io::stdin().read_line(&mut move_input).unwrap();
        if move_input.trim().eq("q") {
            break;
        } else {
            let parsed_move = board.parse_move(&*move_input);
            if parsed_move.is_err() {
                println!("Input is invalid!");
                continue;
            }
            board.move_piece(parsed_move.unwrap().0, parsed_move.unwrap().1);
        }
        print!("{}", board);
    }
    println!("Quitting...");
}
