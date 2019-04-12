/*
The Player acts as the controler of the board.
Since it's checkers there will only be two players in a match.
it will have methods for moving pieces around and attacking.
*/

use piston::input::GenericEvent;

use crate::game_board::Board;

pub struct Player{
    id:u8,
    board: Board,
    pub selected_space: Option<[usize; 2]>,
    cursor_position: [f64;2],
}

impl Player{
    //creates a new player with an id and the board it controls.
    pub fn new(id:u8, board:Board) -> Player{
        Player{id: id, board: board, selected_space: None, cursor_position: [0.0; 2]}
    }

    fn check_move(x1:i32, y1:i32, x2:i32, y2:i32) -> bool{
	       if (x2-x1 == 1 || x2-x1 == -1) && (y2-y1 == 1 || y2-y1 == -1) {
		         true
	       }else{
		         false
           }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E){
        use piston::input::{Button, MouseButton};


        if let Some(pos) = e.mouse_cursor_args(){
            self.cursor_position = pos;
            println!("x:{} y:{}",((self.cursor_position[0]-5.0)/50.0).floor() as usize, (self.cursor_position[1]-5.0)/50.0);
        }

        //TODO clean this up
        //just doing it this way for easy testing
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args(){
            if (((self.cursor_position[0]-5.0)/50.0).floor() as usize)  >= 0 || (((self.cursor_position[0]-5.0)/50.0).floor() as usize) <= 9{
                if (((self.cursor_position[1]-5.0)/50.0) as usize) >= 0 || (((self.cursor_position[1]-5.0)/50.0).floor() as usize) <= 9{
                    self.selected_space = Some([((self.cursor_position[0]-5.0)/50.0).floor() as usize, ((self.cursor_position[1]-5.0)/50.0).floor() as usize]);
                }
            }
        }

    }

    pub fn get_board(&self)->&Board{
        &self.board
    }

    //This moves a piece to an empty spot
    //TODO: check if this spot is either one jump away or not
    pub fn move_piece(&mut self, x:usize, y:usize, x2:usize, y2:usize){
        if let Some(p) = self.board.get_piece(x,y){
            if p.get_player() == self.id{
                if self.board.is_empty(x2, y2){
                    self.board.add_piece(p,x2,y2);
                    self.board.remove_piece(x,y);
                }else{
                    panic!("Can't move to space")
                }
            }
        }else{
            panic!("Player does not own piece.")
        }
    }


}
