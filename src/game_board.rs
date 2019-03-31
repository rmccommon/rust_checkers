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
enum Space {
    Empty,
    NotPlayable,
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

    pub fn setup_board(&mut self){
        //TODO: set up the game pieces for the right players.
    }

    //Checks if a spoace is Empty, used for moving pieces
    pub fn is_empty(&self, x:usize, y:usize) -> bool{
        match self.b_array[y][x]{
            Space::Empty => true,
            _ => false
        }
    }

    //Gets the piece thats on that x,y coordinate
    pub fn get_piece(&self, x:usize, y:usize) -> Piece{
        match &self.b_array[y][x]{
            Space::Full(p) => {return *p;},
            _ => {panic!("No Piece Found!")}
        }
    }

    //adds a piece
    pub fn add_piece(&mut self, p:Piece, x:usize, y:usize){

        match self.b_array[y][x]{
            Space::NotPlayable => {panic!("Can not put piece on a non-playable space!")},
            Space::Full(_) => {panic!("Space is not empty!")},
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

}

//Implements how to display a board if it is in a println! function
impl std::fmt::Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let mut str:String = format!("");
        for row in 0..10{
            for col in 0..10{
                match self.b_array[row][col]{
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
