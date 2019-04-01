/*
The Player acts as the controler of the board.
Since it's checkers there will only be two players in a match.
it will have methods for moving pieces around and attacking.
*/

use crate::piece::Piece;
use crate::game_board::Board;

pub struct Player{
    id:u8,
    board: Board,
}

impl Player{
    //creates a new player with an id and the board it controls.
    pub fn new(id:u8, board:Board) -> Player{
        Player{id, board}
    }

    fn check_move(x1:i32, y1:i32, x2:i32, y2:i32) -> bool{
	if (x2-x1 == 1 || x2-x1 == -1) && (y2-y1 == 1 || y2-y1 == -1) {
		true
	}else{
		false
	}
    }

    //This moves a piece to an empty spot
    //TODO: check if this spot is either one jump away or not
    pub fn move_piece(&mut self, x:usize, y:usize, x2:usize, y2:usize){
        if let p = self.board.get_piece(x,y){
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
