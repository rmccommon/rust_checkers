use crate::game_board::Board;

pub struct Piece {
    player: u8,
    is_king: bool,
    p_id: u32,
}

impl Piece {
    pub fn new(player:u8, count:u32) -> Piece{
        let new_piece = Piece{player:player, is_king:false, p_id:count};
        return new_piece;
    }
    pub fn get_p_id(&self)->u32{
        self.p_id
    }
    pub fn get_player(&self)->u8{
        self.player
    }
}

//These two impl are for making it compatible with the enum in game_board
impl Copy for Piece {}
impl Clone for Piece{
        fn clone(&self)-> Piece{
            *self
        }
}

impl std::fmt::Display for Piece{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Piece: {}, is_king:{}, belongs to player {}", self.p_id, self.is_king,  self.player)
    }
}
