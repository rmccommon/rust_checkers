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


        if let Some(pos) = e.mouse_cursor_args(){
            self.cursor_position = pos;
            println!("x:{} y:{}",((self.cursor_position[0]-5.0)/50.0).floor() as usize, (self.cursor_position[1]-5.0)/50.0);
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
                        }
                    }
                }else if let Some(poss_moves) = &self.possible_moves{

                        for m in poss_moves.iter(){
                            if m[0] == x && m[1] == y{
                                let x2 = m[0];
                                let y2 = m[1];
                                self.board.move_piece(x3, y3, x2, y2);
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
        let mut poss_m: Vec<[usize;2]> = Vec::new();
        let mut attack_ms: Vec<[(usize,usize);2]> = Vec::new();
        //check if a space has been selected
        if let Some(selected) = self.selected_space{
            //check if there is a piece on that space
            let x = selected[0];
            let y = selected[1];
            let left = ((x as isize)-1) as usize;
            let right = x+1;
            if let Some(piece) = self.board.get_piece(x, y){
                //check what side it's on
                if piece.get_player() == 0 {
                    println!("player 1's piece");

                    //simple moves at the moment
                    if self.board.is_empty(((x as isize) - 1) as usize, y+1){
                        poss_m.push([x-1, y+1]);
                    //this is the check for attack moves
                    }else if let Some(piece) = self.board.get_piece(left, y+1){
                        if piece.get_player() == 1 && self.board.is_empty(left-1, y+2){
                            attack_ms.push([(left-1, y+2), (left,y+1)]);
                        }
                    }
                    if self.board.is_empty(right, y+1){
                        poss_m.push([right, y+1]);
                    }else if let Some(piece) = self.board.get_piece(right, y+1){
                        if piece.get_player() == 1 && self.board.is_empty(right+1, y+2){
                            attack_ms.push([(right+1, y+2), (right,y+1)]);
                        }
                    }

                }else{
                    println!("player 2's piece");
                    //Simple moves
                    if self.board.is_empty(((x as isize) - 1) as usize, ((y as isize) - 1) as usize){
                        poss_m.push([x-1, y-1]);
                    }else if let Some(piece) = self.board.get_piece(left, ((y as isize)-1) as usize){
                        if piece.get_player() == 0 && self.board.is_empty(left-1, ((y as isize)-2) as usize){
                            attack_ms.push([(left-1, y-2), (left,y-1)]);
                        }
                    }
                    if self.board.is_empty(x+1, ((y as isize) - 1) as usize){
                        poss_m.push([x+1, y-1]);
                    }else if let Some(piece) = self.board.get_piece(right, ((y as isize)-1) as usize){
                        if piece.get_player() == 0 && self.board.is_empty(right+1, ((y as isize)-2) as usize){
                            attack_ms.push([(right+1, y-2), (right,y-1)]);
                        }
                    }

                }

            }
        }
        if !poss_m.is_empty() && attack_ms.is_empty(){
            self.possible_moves = Some(poss_m);
            self.attack_moves = None;
        }else if !attack_ms.is_empty(){
            self.attack_moves = Some(attack_ms);
            self.possible_moves = None;
        }else{
            self.attack_moves = None;
            self.possible_moves = None;
        }
    }

    pub fn get_board(&self)->&Board{
        &self.board
    }

}
