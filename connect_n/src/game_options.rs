pub struct GameOptions {
    pub board_width: usize,
    pub board_height: usize,
    pub num_to_win: usize
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            board_width: 7, 
            board_height: 6, 
            num_to_win: 4 
        }
    }
}

