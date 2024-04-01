use std::collections::{HashSet};
use grid_2d::{Coord, Grid};

use crate::{game_options::GameOptions, game_result::{GameResult, PieceOrientation, ALL_ORIENTATIONS}, move_result::MoveResult, player::{Player, TeamColor}};

#[derive(PartialEq, Eq)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

pub enum ColumnState {
    Full,
    HasSpaceAtRow {
        row: usize
    }
}

pub struct Game<'a> {
    players: Vec<Player>,
    board: Grid<&'a Player>,
    num_to_win: usize
}

impl<'a> Game<'a> {
    pub fn new(game_options: GameOptions) -> Self {
        Self {
            players: Vec::new(),
            board: Grid::new(game_options.board_width, game_options.board_height),
            num_to_win: game_options.num_to_win
        }
    }

    pub fn make_move(&mut self, player: &Player, column_idx: usize) -> MoveResult {
        
        if let ColumnState::HasSpaceAtRow { row } = self.get_column_state(column_idx) {
            self.board.set(&Coord::new(column_idx, row), player);
        }

        MoveResult::Success
    }
    
    // make more efficient later
    pub fn check_win(&self) -> Option<GameResult> {
        for player in &self.players {
            for coord in &player.occupied_cells {
                for orientation in &ALL_ORIENTATIONS {
                    let (dir1, dir2) = orientation.into_directions();
                    let (coords_in_dir1, coords_in_dir2) = (
                        self.march(coord, player.team_color, dir1),
                        self.march(coord, player.team_color, dir2),
                    );
                    if coords_in_dir1.len() + coords_in_dir2.len() - 1 >= self.num_to_win {
                        let mut winning_coords: HashSet<Coord> = HashSet::new();
                        winning_coords.extend(coords_in_dir1);
                        winning_coords.extend(coords_in_dir2);
                        return Some(
                            GameResult::Won {
                                team_color: player.team_color, 
                                orientation: *orientation, 
                                winning_coords
                            }
                        )
                    }
                }
            }
        }

        None
    }

    fn get_column_state(&self, column_idx: usize) -> ColumnState {

    }

    // returns longest chain found in direction (in coord vec)
    // INCLUDES STARTER COORD
    fn march(&self, start_coord: &Coord, for_team: TeamColor, direction: Direction) -> Vec<Coord> {
        let mut coord_vec = vec![start_coord.clone()];

        let (shift_x, shift_y) = match direction {
            Direction::Top => (0, -1),
            Direction::Bottom => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::TopLeft => (-1, -1),
            Direction::TopRight => (1, -1),
            Direction::BottomLeft => (-1, 1),
            Direction::BottomRight => (1, 1),
        };

        let mut coord_to_check = start_coord.translated(shift_x, shift_y);

        loop {
            match coord_to_check {
                Some(c) => {
                    let team = self.board.get(&c);

                    match team {
                        Some(t) => {
                            if t.team_color != for_team {
                                return coord_vec;
                            }

                            coord_vec.push(c);
                        },
                        None => return coord_vec,
                    }
                    
                    coord_to_check = c.translated(shift_x, shift_y);
                },
                None => return coord_vec,
            }
        }
    }
}