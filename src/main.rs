use crossterm::event::{read, Event, KeyEventKind, KeyCode, poll};
use crossterm::terminal::ClearType;
use crossterm::{event, execute, terminal};
use std::{thread::sleep, time::Duration};
use std::io::{stdout, Write};


#[derive(Debug)]
struct Cursor{
    x: usize,
    y: usize
}

impl Cursor{
    fn new() -> Self{
        Self{
            x: 0,
            y: 0
        }
    }
    pub fn goto(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

}

#[derive(Debug)]
struct Game{
    grid: [[char; 3]; 3],
    cursor: Cursor,
    prev: Cursor
}


impl Game{
    fn new() -> Self{
        Self{
            grid: [[' '; 3]; 3],
            cursor: Cursor::new(),
            prev: Cursor::new()
            
        }
    }
}



fn main(){
    let mut game = Game::new();    
    game.cursor.goto(1, 2);
    draw_game(&mut game);

}


fn draw_game(g: &mut Game){





    loop{

        execute!(stdout(), terminal::Clear(ClearType::All));

        g.grid[g.prev.y][g.prev.x] = ' ';

        handle_inputs(&mut g.cursor);

        g.grid[g.cursor.y][g.cursor.x] = '_';

        g.prev.x = g.cursor.x;
        g.prev.y = g.cursor.y;



        for rows in g.grid{
            println!("\n{:?}", rows);
        }
        
        sleep(Duration::from_millis(10));

    }

}


fn handle_inputs(cursor: &mut Cursor) {

    terminal::enable_raw_mode();

    if poll(Duration::from_millis(1)).unwrap() {
        if let Event::Key(event) = read().unwrap() {
            if event.kind == KeyEventKind::Press {
                match event.code {
                    KeyCode::Up => {
                        if cursor.y > 0 {             //tuka ima out of bounds fixni go tupak
                            cursor.y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if cursor.y < 2 {
                            cursor.y += 1;
                        }
                    }
                    KeyCode::Right => {
                        if cursor.x < 2 {
                            cursor.x += 1;
                        }
                    }
                    KeyCode::Left => {
                        if cursor.x > 0 {
                            cursor.x -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode();
}