use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};
use crossterm::terminal::ClearType;
use crossterm::{execute, terminal};
use std::io::stdout;
use std::{thread::sleep, time::Duration};

mod game;
use game::Cursor;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.cursor.goto(1, 2);
    draw_game(&mut game);
}

fn draw_game(g: &mut Game) {
    loop {
        execute!(stdout(), terminal::Clear(ClearType::All)).expect("failed to clear :(");

        g.grid[g.prev.y][g.prev.x] = ' ';

        handle_inputs(&mut g.cursor);

        g.grid[g.cursor.y][g.cursor.x] = '_';

        g.prev.x = g.cursor.x;
        g.prev.y = g.cursor.y;

        for rows in g.grid {
            println!("\n{:?}", rows);
        }

        sleep(Duration::from_millis(10));
    }
}

fn handle_inputs(cursor: &mut Cursor) {
    if let Err(e) = terminal::enable_raw_mode() {
        panic!("{}", e);
    }

    if poll(Duration::from_millis(1)).unwrap() {
        if let Event::Key(event) = read().unwrap() {
            if event.kind == KeyEventKind::Press {
                match event.code {
                    KeyCode::Up => {
                        if cursor.y > 0 {
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
    if let Err(e) = terminal::disable_raw_mode() {
        panic!("{}", e);
    }
}
