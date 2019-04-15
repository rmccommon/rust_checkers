/*
The Player acts as the controler of the board.
Since it's checkers there will only be two players in a match.
it will have methods for moving pieces around and attacking.
*/

use crate::piece::Piece;
use piston::input::GenericEvent;

use crate::game_board::Board;

pub struct Player{
    pub id:u8,
    board: Board,
    pub selected_space: Option<[usize; 2]>,
    pub possible_moves: Option<Vec<[usize; 2]>>,

    //attack moves consist of [(destination_x, destination_y),(attacked_piece_x, attacked_piece_y)]
    pub attack_moves: Option<Vec<[(usize,usize); 2]>>,

    cursor_position: [f64;2],
}

impl Player{
    //creates a new player with an id and the board it controls.
    pub fn new(id:u8, board:Board) -> Player{
        Player{id: id, board: board, selected_space: None, possible_moves: None, attack_moves:None, cursor_position: [0.0; 2]}
    }


    pub fn event<E: GenericEvent>(&mut self, e: &E){
        use piston::input::{Button, MouseButton};
        let switch_turn = |player:u8| {
            if player == 1{
                0
            }else{
                1
            }
        };


        if let Some(pos) = e.mouse_cursor_args(){
            self.cursor_position = pos;
        }

        //TODO clean this up
        //just doing it this way for easy testing
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args(){
            let x = ((self.cursor_position[0]-5.0)/50.0).floor() as usize;
            let y = ((self.cursor_position[1]-5.0)/50.0).floor() as usize;

            if let Some(selected) = self.selected_space{
                let x3 = selected[0];
                let y3 = selected[1];
                if let Some(attack_ms) = &self.attack_moves{
                    for a in attack_ms.iter(){
                        if a[0].0 == x && a[0].1 == y{
                            self.board.remove_piece(a[1].0, a[1].1);
                            self.board.move_piece(x3, y3, a[0].0, a[0].1);
                            self.id = switch_turn(self.id);
                        }
                    }
                }else if let Some(poss_moves) = &self.possible_moves{

                        for m in poss_moves.iter(){
                            if m[0] == x && m[1] == y{
                                let x2 = m[0];
                                let y2 = m[1];
                                self.board.move_piece(x3, y3, x2, y2);
                                self.id = switch_turn(self.id);
                            }
                        }
                }
            }

            if x >= 0 && x <= 9 && y >= 0 && y<=9 {
                self.selected_space = Some([x,y]);
                self.find_moves();
            }
        }

    }


    fn find_moves(&mut self){
        //closure to find simple moves(Non-attack moves)
        let move_decider = |x:usize,y:usize, piece:Piece|{
            let mut possible_moves:Vec<[usize;2]> = Vec::new();
            //possible directions it can move
            let left = ((x as isize) - 1) as usize;
            let right = x + 1;
            let up = ((y as isize) - 1) as usize;
            let down = y+1;
            //push them on to a vector
            let mut x_dir:Vec<usize> = Vec::new();
            x_dir.push(left);
            x_dir.push(right);

            //check which direction it can go up, down, or both
            let mut y_dir:Vec<usize> = Vec::new();
            if piece.is_king() {
                y_dir.push(up);
                y_dir.push(down);
            }else if piece.get_player() == 0{
                y_dir.push(down);
            }else{
                y_dir.push(up);
            }
            //go through all the directions and check if theres a move there
            for poss_x in x_dir.iter(){
                for poss_y in y_dir.iter(){
                    if self.board.is_empty(*poss_x, *poss_y){
                        //push the move on the vector
                        possible_moves.push([*poss_x, *poss_y]);
                    }

                }
        }

            if possible_moves.is_empty(){
                None
            }else{
                Some(possible_moves)
            }
        };
        //closure to find simple moves upward
        let move_up = |x:usize, y:usize|{
        };
        let mut poss_m: Option<Vec<[usize;2]>> = None;
        let mut attack_ms: Vec<[(usize,usize);2]> = Vec::new();
        //check if a space has been selected
        if let Some(selected) = self.selected_space{
            //check if there is a piece on that space
            let x = selected[0];
            let y = selected[1];
            if let Some(piece) = self.board.get_piece(x, y){
                if piece.get_player() != self.id{
                    self.possible_moves = None;
                    self.attack_moves = None;
                    return;
                }
                //simple moves at the moment
                poss_m = move_decider(x,y,piece);
            }
        }
        self.possible_moves = poss_m;
    }

    //returns a reference to the board so the view can see it
    pub fn get_board(&self)->&Board{
        &self.board
    }

}
