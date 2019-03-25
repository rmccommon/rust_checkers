/*
This modual will modal the checkers game board.
It will have functions to create a new board.
reset/set up the board.
And whatever else it needs to do, that I can think of right now.
*/
use crate::piece;

#[derive(Clone, Copy)]
enum Space {
    empty,
    not_playable,
    full(u32),
}

pub struct Board {
    b_array: [[Space; 10]; 10],
}

impl Board{

    pub fn new()->Board{
        let mut new_p_board:[[Space; 10]; 10] = [[Space::not_playable; 10]; 10];
        for row in 0..10{
            if row%2 == 0{
                for col in 0..10{
                    if col%2 == 0{
                            new_p_board[row][col] = Space::empty;
                    }
                }
            }else{
                for col in 0..10{
                    if col%2 == 1{
                        new_p_board[row][col] = Space::empty;
                    }
                }
            }
        }
        return Board{b_array: new_p_board};
    }
}
