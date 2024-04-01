use grid_2d::Coord;

pub struct Player {
    pub team_color: TeamColor,
    pub occupied_cells: Vec<Coord>
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TeamColor {
    Red,
    Yellow
}