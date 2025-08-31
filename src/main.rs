use inputbot;
use std::{thread::sleep, time::Duration};
use std::io::Write;

fn main() {


    draw_game();
    
    
}


fn draw_game(){

    let mut cursor: (usize, usize) =(0, 0);

    let mut grid:[[char; 3]; 3] = 
    [[' ', ' ', ' '],
     [' ', ' ', ' '],
     [' ', ' ', ' ']
    ];

    //infinite loop
    loop{

        grid[cursor.0][cursor.1] = '_';

        for row in grid{
        println!("{:?}", row)
        }

        sleep(Duration::from_millis(1000));


        clear_console();

    }

}

fn clear_console() {
    // The ANSI escape code for clearing the terminal screen
    print!("\x1B[2J\x1B[1;1H");
    // Flush the output to ensure the console is cleared immediately
    std::io::stdout().flush().unwrap();
}