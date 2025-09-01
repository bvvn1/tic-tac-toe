#[derive(Debug)]
pub struct Game {
    pub grid: [[char; 3]; 3],
    pub cursor: Cursor,
    pub prev: Cursor,
}

#[derive(Debug)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn goto(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: [[' '; 3]; 3],
            cursor: Cursor::new(),
            prev: Cursor::new(),
        }
    }
}
