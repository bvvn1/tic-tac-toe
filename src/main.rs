use crossterm::{
    execute,
    style::Stylize,
    terminal::{self, ClearType},
};
use std::io::stdout;
use std::{thread::sleep, time::Duration};

use crate::game::{TicTacToe, handle_inputs};
pub mod game;

fn game() {
    let g: &mut TicTacToe = &mut TicTacToe::new();

    loop {
        execute!(stdout(), terminal::Clear(ClearType::All)).expect("those who know");
        handle_inputs(g);

        for (i, row) in g.board.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if (i, j) == (g.pointer.y, g.pointer.x) {
                    print!("|{}|", col.to_str().on_red())
                } else {
                    print!("|{}|", col.to_str())
                }
                // print!("{:?}", g.pointer);
            }
            println!();
        }

        sleep(Duration::from_millis(10));

        if g.check_winner() == Some(game::Cell::O) {
            println!("O wins!");
            break;
        } else if g.check_winner() == Some(game::Cell::X) {
            println!("X wins!");
            break;
        } else if g.is_draw() == true {
            println!("Draw!");
            break;
        }
    }
}

fn main() {
    game();
}
