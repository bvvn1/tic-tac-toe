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

    draw_game(&mut game);
}

fn draw_game(g: &mut Game) {
    loop {
        execute!(stdout(), terminal::Clear(ClearType::All)).expect("failed to clear :(");
        if g.grid[g.prev.y][g.prev.x] == '_' {
            g.grid[g.prev.y][g.prev.x] = ' ';
        }
        handle_inputs(g);
        if g.grid[g.cursor.y][g.cursor.x] == 'x' {
            g.grid[g.cursor.y][g.cursor.x] = 'Ⓧ';
        } else if g.grid[g.cursor.y][g.cursor.x] == 'y' {
            g.grid[g.cursor.y][g.cursor.x] = 'Ⓨ';
        } else {
            g.grid[g.cursor.y][g.cursor.x] = '_';
        }

        g.prev.x = g.cursor.x;
        g.prev.y = g.cursor.y;

        for rows in &g.grid {
            println!("\n{:?}", rows);
        }

        sleep(Duration::from_millis(10));
    }
}

fn handle_inputs(g: &mut Game) {
    let turn = false;

    if let Err(e) = terminal::enable_raw_mode() {
        panic!("{}", e);
    }

    if poll(Duration::from_millis(1)).unwrap() {
        if let Event::Key(event) = read().unwrap() {
            if event.kind == KeyEventKind::Press {
                match event.code {
                    KeyCode::Up => {
                        if g.cursor.y > 0 {
                            g.cursor.y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if g.cursor.y < 2 {
                            g.cursor.y += 1;
                        }
                    }
                    KeyCode::Right => {
                        if g.cursor.x < 2 {
                            g.cursor.x += 1;
                        }
                    }
                    KeyCode::Left => {
                        if g.cursor.x > 0 {
                            g.cursor.x -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        if turn == false {
                            g.grid[g.cursor.x][g.cursor.y] = 'x';
                            // g.grid_without_cursor[g.cursor.x][g.cursor.y] = 'x';
                            turn == true;
                        }
                        if turn == true {
                            g.grid[g.cursor.x][g.cursor.y] = 'y';
                            // g.grid_without_cursor[g.cursor.x][g.cursor.y] = 'y';
                            turn == false;
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
