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


    

    //infinite loop
    loop{


        handle_inputs(cursor);

        grid[cursor.0][cursor.1] = '_';

        print!("{:?}", cursor);

         
        for row in &grid{
        println!("{:?}", row)
        }
        
        sleep(Duration::from_millis(1000));

        execute!(stdout(), terminal::Clear(ClearType::All));
    

    }

}


fn handle_inputs(cursor: &mut (usize, usize)) {
    


}