/*
This modual will model the checkers game board.
It will have functions to create a new board.
reset/set up the board.
Get pieces on the board.
Add/Remove pieces.
*/
use crate::piece::Piece;


//This is the enum for what a space on the board can be
//Rust has a nice feature where an enum can hold a struct, like piece.
#[derive(Clone, Copy)]
pub enum Space {
    Empty,
    NotPlayable,
    Dummy,
    Full(Piece),
}

//The board is just a 2d array of spaces
pub struct Board {
    b_array: [[Space; 10]; 10],
}

impl Board{
    //This sets up an empty checkers board
    pub fn new()->Board{
        let mut new_p_board:[[Space; 10]; 10] = [[Space::NotPlayable; 10]; 10];
        for row in 0..10{
            if row%2 == 0{
                for col in 0..10{
                    if col%2 == 0{
                            new_p_board[row][col] = Space::Empty;
                    }
                }
            }else{
                for col in 0..10{
                    if col%2 == 1{
                        new_p_board[row][col] = Space::Empty;
                    }
                }
            }
        }
        return Board{b_array: new_p_board};
    }

    pub fn place_dummy(&mut self, x: usize, y: usize) {
        self.b_array[y][x] = Space::Dummy;
    }

    pub fn setup_board(&mut self){
        //TODO: set up the game pieces for the right players.
        for i in 0..4{
            for j in 0..10{
                match self.b_array[i][j]{
                    Space::Empty => {self.b_array[i][j] = Space::Full(Piece::new(0));},
                    _ => {}
                }
            }
        }
        for i in 6..10{
            for j in 0..10{
                match self.b_array[i][j]{
                    Space::Empty => {self.b_array[i][j] = Space::Full(Piece::new(1));},
                    _ => {}
                }
            }
        }



    }
    pub fn get_space(&self, x:usize, y:usize) -> Space{
        self.b_array[y][x]
    }

    //Checks if a space is Empty, used for moving pieces
    pub fn is_empty(&self, x:usize, y:usize) -> bool{
        //This is to check for out of bounds
        if x >= 10 || y >= 10{
            return false;
        }
        match self.b_array[y][x]{
            Space::Empty => true,
            _ => false
        }
    }

    //Gets the piece thats on that x,y coordinate
    pub fn get_piece(&self, x:usize, y:usize) -> Option<Piece>{
        if x>=10 || y >= 10{
            return None;
        }
        match &self.b_array[y][x]{
            Space::Full(p) => {Some(*p)},
            _ => {None}
        }
    }

    //adds a piece
    pub fn add_piece(&mut self, p:Piece, x:usize, y:usize){

        match self.b_array[y][x]{
            Space::NotPlayable => {panic!("Can not put piece on a non-playable space!")},
            Space::Full(_) => {panic!("Space is not empty!")},
            Space::Dummy => {panic!("DUMMY SHOULD BE REMOVED YOU DUMMY");},
            Space::Empty => {self.b_array[y][x] = Space::Full(p);}
        }
    }

    //removes a piece
    pub fn remove_piece(&mut self, x:usize, y:usize){
        match self.b_array[y][x]{
            Space::Full(_) => {self.b_array[y][x] = Space::Empty;},
            _ => {panic!("No piece on space!")},
        }
    }

    pub fn move_piece(&mut self, x:usize, y:usize, x2:usize, y2:usize){
            if let Some(mut piece) = self.get_piece(x, y){
                if piece.get_player() == 0 && y2 == 9{
                    piece.king_me();
                }
                if piece.get_player() == 1 && y2 == 0{
                    piece.king_me();
                }
                self.add_piece(piece, x2, y2);
                self.remove_piece(x, y);
            }
    }
    pub fn remove_dummies(&mut self){
        for y in 0..10{
            for x in 0..10{
                match self.b_array[x][y] {
                    Space::Dummy => {self.b_array[x][y] = Space::Empty;},
                    _ => {}
                }
            }
        }
    }



}



//Implements how to display a board if it is in a println! function
impl std::fmt::Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let mut str:String = format!("");
        for row in 0..10{
            for col in 0..10{
                match self.b_array[row][col]{
                    Space::Dummy => {panic!("DUMMY SHOULD BE REMOVED YOU DUMMY");},
                    Space::NotPlayable => {str = format!("{}[X]", str);},
                    Space::Empty => {str = format!("{}[ ]", str);},
                    Space::Full(p) => {str = format!("{}[{}]",str, p.get_player());},
                }
            }
            str = format!("{}\n", str);
        }
        write!(f, "{}", str)
    }
}
