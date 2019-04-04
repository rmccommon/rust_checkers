extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
use piston::window::WindowSettings;
use piston::event_loop::{Events,EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

//! View for the game and its logic
use crate::game_board;

pub stuct game_view{
    board: Board,
}
