/*
Project: Rust Checkers
Authors: Ryan McCommon, Dylan Senderling

This will use an MVC Design pattern, with the game board being the model,
the player as the controller, and piston as the view.
Main will mostly be the games event loop.
*/
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
mod game;
mod player;



fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Checkers", [512;2]).opengl(opengl).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not make window");

    let  mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let mut board1 = game_board::Board::new();
    board1.setup_board();
    let g_view = game::game_view::new(500.0);
    let mut player1 = player::Player::new(0, board1);

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c,g| {
                use graphics::clear;

                clear([1.0; 4], g);
                g_view.draw(&player1, &c, g);
            })
        }

    }


}
