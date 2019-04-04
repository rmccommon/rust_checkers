use crate::piece;
use crate::game_board;

pub struct Player{
    id:u8,
    g_board:Board,
}

impl Player{
    pub fn new(id:8, g_board:Board) -> Player{
        Player{id,g_board}
    }


}
