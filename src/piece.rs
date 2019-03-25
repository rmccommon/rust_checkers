use crate::game_board::Board;

pub struct Piece {
    player: u8,
    x: u32,
    y: u32,
    is_dead: bool,
    is_king: bool,
    p_id: u32,
    board: Board,
}

impl Piece {
    pub fn new(player:u8, x:u32, y:u32,count:u32, board: Board) -> Piece{
        let new_piece = Piece{player:player, x:x, y:y, is_dead:false, is_king:false,p_id:count,board:board};
        return new_piece;
    }
}
impl std::fmt::Display for Piece{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Piece: {}, is_king:{}, is_dead: {}, belongs to player {} and is located at x:{} y:{}", self.p_id, self.is_king, self.is_dead, self.player, self.x, self.y)
    }
}
