/*
This is the view for the game
it is responsible for drawing the board and its pieces
*/


use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::game_board::{Board, Space};
use crate::player::Player;
use crate::piece::Piece;

pub struct GameView{
    board_size: f64,
    background_color: Color,
}

impl GameView{

    pub fn new(size:f64,)-> GameView{
        GameView{board_size:size, background_color:[1.0,1.0,1.0,1.0]}
    }

    //This is the function that draws the checkers board and its pieces
    pub fn draw<G:Graphics>(&self, controller: &Player, c: &Context, g: &mut G){
        let p1 = 0;
        let p2 = 1;
        use graphics::{Rectangle,Ellipse};

        //this is the starting location of the board in the Window
        //(0, 0) is the upper left corner of the window
        let (start_x, start_y) = (5.0, 5.0);

        //Colors of the spaces on the Board
        //[red, green, blue, alpha]
        let red:Color = [1.0,0.0,0.0,1.0];
        let black:Color = [0.0,0.0,0.0,1.0];
        let yellow:Color = [1.0,1.0,0.0,0.10];
        let bluish:Color = [66.0/255.0, 140.0/255.0, 1.0, 0.25];
        let green:Color = [0.0, 1.0, 0.0, 0.60];

        //Colors of the players pieces
        let p1_color = [153.0/255.0, 0.0, 0.0, 1.0];
        let p2_color = [17.0/255.0, 17.0/255.0, 9.0/255.0, 1.0];

        //size of a space
        let square_size = self.board_size/10.0;

        //creats a rectangle object with the color red
        let space_playable = Rectangle::new(red);
        //creates a rectangle object that is black
        let space_unplayable = Rectangle::new(black);
        let p1_piece = Ellipse::new(p1_color);
        let p2_piece = Ellipse::new(p2_color);
        let circle_offset = square_size/10.0;

        //gets the board from the controller
        //this will probably change in the future since it's not exactly MVC
        let board = controller.get_board();

        //Go through the board and draw every space
        //if there is a piece draw that too
        for i in 0..10{
            for j in 0..10{
                //creates the dimensions and position of where we want to draw a shape
                let rect = [((j as f64)*square_size)+start_x, ((i as f64)*square_size)+start_y, square_size, square_size];
                let circ = [((j as f64)*square_size)+start_x+circle_offset, ((i as f64)*square_size)+start_y+circle_offset, square_size/1.25, square_size/1.25];

                match board.get_space(j, i){
                    Space::Empty => {space_playable.draw(rect, &c.draw_state, c.transform, g);},

                    Space::NotPlayable => {space_unplayable.draw(rect, &c.draw_state, c.transform, g);},


                    Space::Full(p) => {space_playable.draw(rect, &c.draw_state, c.transform, g);
                                        if p.get_player() == p1{
                                                p1_piece.draw(circ, &c.draw_state, c.transform, g);
                                        }else{
                                                p2_piece.draw(circ, &c.draw_state, c.transform, g);
                                        }
                                    }
                }
            }
        }

        let space_selected = Rectangle::new(yellow);
        let attack_space = Rectangle::new(green);
        let possible_space = Rectangle::new(bluish);
        if let Some(select_square) = controller.selected_space {
            let rect = [((select_square[0] as f64)*square_size)+start_x,((select_square[1] as f64)*square_size)+start_y, square_size, square_size];
            space_selected.draw(rect, &c.draw_state, c.transform, g);
        }

        if let Some(attack_ms) = &controller.attack_moves{
            for a in attack_ms.iter(){
                let rect = [((a[0].0 as f64)*square_size)+start_x,((a[0].1 as f64)*square_size)+start_y, square_size, square_size];
                attack_space.draw(rect, &c.draw_state, c.transform, g);
            }
        }else if let Some(poss_moves) = &controller.possible_moves{
            for m in poss_moves.iter(){
                let rect = [((m[0] as f64)*square_size)+start_x,((m[1] as f64)*square_size)+start_y, square_size, square_size];
                possible_space.draw(rect, &c.draw_state, c.transform, g);
            }
        }



    }

}
