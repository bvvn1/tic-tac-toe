
#[derive(Debug)]
struct Cursor{
    x: i8,
    y: i8
}

impl Cursor{
    fn new() -> Self{
        Self{
            x: 0,
            y: 0
        }
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
    
}

