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
mod player;



fn main() {
    let board1 = game_board::Board::new();
    let piece1 = piece::Piece::new(0);
    println!("{}", board1);
}
