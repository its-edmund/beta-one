use board::Board;
use board::Square;
use piece::Blank;
use piece::Piece;
use std::io::{self, Write};

pub mod board;
pub mod piece;

fn main() {
    let mut board = Board::from_fen(String::from(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
    ));
    print!("{}", board);

    let mut move_input = String::new();
    loop {
        move_input.clear();
        io::stdout().flush();
        std::io::stdin().read_line(&mut move_input).unwrap();
        if move_input.trim().eq("q") {
            break;
        } else {
            let parsed_move = Board::parse_move(&*move_input);
            let check_color = board.check_color_piece(parsed_move.unwrap().0);

            if check_color {
                board.toggle_current_move();
                board.move_piece(parsed_move.unwrap().0, parsed_move.unwrap().1);
            } else {
                println!("Correct color must move!");
            }

            if parsed_move.is_err() {
                println!("Input is invalid!");
                continue;
            }
        }
        print!("{}", board);
    }
    println!("Quitting...");
}
