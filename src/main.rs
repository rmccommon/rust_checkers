extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
use piston::window::WindowSettings;
use piston::event_loop::{Events,EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};



mod piece;
mod game_board;



fn main() {
    let board1 = game_board::Board::new();
    let piece1 = piece::Piece::new(0, 0);


    //code for piston will probably move out of main function once I get it working
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Checkers", [512;2]).opengl(opengl).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not make window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){
            gl.draw(args.viewport(), |c,g| {
                use graphics::*;

                clear([1.0;4], g);
            });
        }
    }

    println!("{}", board1);
}
