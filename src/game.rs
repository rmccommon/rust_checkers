
use graphics::types::Color;
use graphics::{Context, Graphics};
// View for the game and its logic
use crate::game_board::{Board, Space};
use crate::player::Player;

pub struct game_view{
    window_size: f64,
    background_color: Color,
}

impl game_view{
    pub fn new(size:f64,)-> game_view{
        game_view{window_size:size, background_color:[1.0,1.0,1.0,1.0]}
    }

    pub fn draw<G:Graphics>(&self, controller: &Player, c: &Context, g: &mut G){
        use graphics::{Rectangle,Ellipse};

        let (start_x, start_y) = (self.window_size/16.0, 10.0);

        let red:Color = [1.0,0.0,0.0,1.0];
        let black:Color = [0.0,0.0,0.0,1.0];

        let square_size = self.window_size/10.0;

        let board_background = Rectangle::new(self.background_color).draw([0.0,0.0,self.window_size,self.window_size], &c.draw_state, c.transform, g);
        let space_playable = Rectangle::new(red);
        let space_unplayable = Rectangle::new(black);
        let piece = Ellipse::new(black);
        let board = controller.get_board();


        for i in 0..9{
            for j in 0..9{
                let rect = [((i as f64)*square_size)+start_x,((j as f64)*square_size)+start_y,square_size,square_size];
                match board.get_space(j, i){
                    Space::Empty => {space_playable.draw(rect, &c.draw_state, c.transform, g);},
                    Space::NotPlayable => {space_unplayable.draw(rect, &c.draw_state, c.transform, g);},
                    Space::Full(_) => {space_playable.draw(rect, &c.draw_state, c.transform, g);
                                        piece.draw(rect, &c.draw_state, c.transform, g);}
                }
            }
        }
    }

}
