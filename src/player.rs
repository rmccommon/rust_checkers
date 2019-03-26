use crate::piece;
use crate::board;

pub struct Player{
    id:u8,
}

impl Player{
    pub fn new(id:u32) -> Player{
        Player{id}
    }
}
