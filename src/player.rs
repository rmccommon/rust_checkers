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
    pub possible_moves: Option<Vec<[usize; 2]>>,
    cursor_position: [f64;2],
}

impl Player{
    //creates a new player with an id and the board it controls.
    pub fn new(id:u8, board:Board) -> Player{
        Player{id: id, board: board, selected_space: None, possible_moves: None, cursor_position: [0.0; 2]}
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
            let x = ((self.cursor_position[0]-5.0)/50.0).floor() as usize;
            let y = ((self.cursor_position[1]-5.0)/50.0).floor() as usize;

            if let Some(poss_moves) = &self.possible_moves{
                let x3 = self.selected_space.unwrap()[0];
                let y3 = self.selected_space.unwrap()[1];
                    for m in poss_moves.iter(){
                        if m[0] == x && m[1] == y{
                            let x2 = m[0];
                            let y2 = m[1];
                            self.board.move_piece(x3, y3, x2, y2);
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
        let mut poss_m: Vec<[usize;2]> = Vec::new();
        //check if a space has been selected
        if let Some(selected) = self.selected_space{
            //check if there is a piece on that space
            let x = selected[0];
            let y = selected[1];
            if let Some(piece) = self.board.get_piece(x, y){
                //check what side it's on
                if piece.get_player() == 0 {
                    println!("player 1's piece");

                    //simple moves at the moment
                    if self.board.is_empty(((x as isize) - 1) as usize, y+1){
                        poss_m.push([x-1, y+1]);
                    }
                    if self.board.is_empty(x+1, y+1){
                        poss_m.push([x+1, y+1]);
                    }

                }else{
                    println!("player 2's piece");
                    //Simple moves
                    if self.board.is_empty(((x as isize) - 1) as usize, ((y as isize) - 1) as usize){
                        poss_m.push([x-1, y-1]);
                    }
                    if self.board.is_empty(x+1, ((y as isize) - 1) as usize){
                        poss_m.push([x+1, y-1]);
                    }

                }

            }
        }
        if !poss_m.is_empty(){
            self.possible_moves = Some(poss_m);
        }else{
            self.possible_moves = None;
        }
    }

    pub fn get_board(&self)->&Board{
        &self.board
    }

    //This moves a piece to an empty spot
    //TODO: check if this spot is either one jump away or not
    pub fn move_piece(&mut self, x2:usize, y2:usize){
        if let Some(selected) = self.selected_space{
            if let Some(piece) = self.board.get_piece(selected[0], selected[1]){
                self.board.add_piece(piece, x2, y2);
                self.board.remove_piece(selected[0], selected[1]);
            }
        }
    }


}
