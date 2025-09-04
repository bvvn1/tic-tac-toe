use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, KeyEventKind, poll, read},
    terminal,
};

#[derive(Clone, Copy, PartialEq)]
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

    pub fn check_winner(&self) -> Option<Cell> {
        for i in 0..3 {
            if self.board[i][0] != Cell::Empty
                && self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2]
            {
                return Some(self.board[i][0]);
            }
        }
        for j in 0..3 {
            if self.board[0][j] != Cell::Empty
                && self.board[0][j] == self.board[1][j]
                && self.board[1][j] == self.board[2][j]
            {
                return Some(self.board[0][j]);
            }
        }

        if self.board[0][0] != Cell::Empty
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
        {
            return Some(self.board[0][0]);
        }
        if self.board[0][2] != Cell::Empty
            && self.board[1][1] == self.board[0][2]
            && self.board[1][1] == self.board[2][0]
        {
            return Some(self.board[0][2]);
        }

        None
    }

    pub fn is_draw(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|c| *c != Cell::Empty))
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
                        if g.board[g.pointer.y][g.pointer.x] == Cell::Empty {
                            if g.turn == false {
                                g.board[g.pointer.y][g.pointer.x] = Cell::X;
                                g.turn = true;
                            } else if g.turn == true {
                                g.board[g.pointer.y][g.pointer.x] = Cell::O;
                                g.turn = false;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode().expect("baba ti")
}
