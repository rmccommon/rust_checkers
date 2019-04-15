/*
This object represents the game piece.
It keeps track of who it is owned by.
If its a king or not.
*/

pub struct Piece {
    player: u8,
    is_king: bool,
}


impl Piece {
    //create a new piece and assigns an owner
    pub fn new(player:u8) -> Piece{
        Piece{player:player, is_king:false}
    }

    //gets king status
    pub fn is_king(&self) -> bool{
        self.is_king
    }

    pub fn king_me(&mut self){
        if self.is_king == false{
            self.is_king = true;
        }
    }

    //gets the player id that owns it
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

//This is mostly for debug purposes but
//it implements a way for println! to display a piece object
impl std::fmt::Display for Piece{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "This is a game piece, is_king:{}, belongs to player {}",  self.is_king,  self.player)
    }
}
