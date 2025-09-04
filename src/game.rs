use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, KeyEventKind, poll, read},
    terminal,
};

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    X,
    O,
}
impl Cell {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Empty => " ",
            Self::O => "O",
            Self::X => "X",
        }
    }
}
#[derive(Debug)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub struct TicTacToe {
    pub board: [[Cell; 3]; 3],
    pub pointer: Cursor,
    pub turn: bool,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [
                [Cell::Empty, Cell::Empty, Cell::Empty],
                [Cell::Empty, Cell::Empty, Cell::Empty],
                [Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            pointer: Cursor::new(),
            turn: false,
        }
    }
}

pub fn handle_inputs(g: &mut TicTacToe) {
    terminal::enable_raw_mode().expect("maika ti");

    if poll(Duration::from_millis(1)).unwrap() {
        if let Event::Key(event) = read().unwrap() {
            if event.kind == KeyEventKind::Press {
                match event.code {
                    KeyCode::Up => {
                        if g.pointer.y > 0 {
                            g.pointer.y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if g.pointer.y < 2 {
                            g.pointer.y += 1;
                        }
                    }
                    KeyCode::Right => {
                        if g.pointer.x < 2 {
                            g.pointer.x += 1;
                        }
                    }
                    KeyCode::Left => {
                        if g.pointer.x > 0 {
                            g.pointer.x -= 1;
                        }
                    }
                    KeyCode::Char(' ') => {
                        if g.turn == false {
                            g.board[g.pointer.y][g.pointer.x] = Cell::X;
                            g.turn = true;
                        } else if g.turn == true {
                            g.board[g.pointer.y][g.pointer.x] = Cell::O;
                            g.turn = false;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode().expect("baba ti")
}
