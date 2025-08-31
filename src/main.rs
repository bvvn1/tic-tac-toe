use crossterm::event::{read, Event, KeyEventKind, KeyCode, poll};
use crossterm::terminal::ClearType;
use crossterm::{event, execute, terminal};
use std::{thread::sleep, time::Duration};
use std::io::{stdout, Write};






fn main() {
    let mut cursor: (usize, usize) =(0, 0);
    if cursor.0 > 3 || cursor.1 > 3 {
        panic!("Out of bounds");
    }
    
    draw_game(&mut cursor);
    


   
    
}

fn draw_game(cursor: &mut (usize, usize)){



    let mut grid:[[char; 3]; 3] = 
    [[' ', ' ', ' '],
     [' ', ' ', ' '],
     [' ', ' ', ' ']
    ];
    let mut prev = *cursor;

    

    //infinite loop
    loop{


        execute!(stdout(), terminal::Clear(ClearType::All));


        grid[prev.0][prev.1] = ' ';

        handle_inputs(cursor);

        grid[cursor.0][cursor.1] = '_';
        
        prev = *cursor;

         
        for row in &grid{
        println!("{:?}", row)
        }
        
        sleep(Duration::from_millis(1000));

        
    

    }

}

fn handle_inputs(cursor: &mut (usize, usize)) {
    
    terminal::enable_raw_mode();

    if poll(Duration::from_millis(100)).unwrap() {
        if let Event::Key(event) = read().unwrap() {
            if event.kind == KeyEventKind::Press {
                match event.code {
                    KeyCode::Up => {
                        if cursor.0 > 0 {
                            cursor.0 -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if cursor.0 < 2 {
                            cursor.0 += 1;
                        }
                    }
                    KeyCode::Right => {
                        if cursor.1 < 2 {
                            cursor.1 += 1;
                        }
                    }
                    KeyCode::Left => {
                        if cursor.1 > 0 {
                            cursor.1 -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode();
}


