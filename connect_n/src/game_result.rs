use std::collections::HashSet;

use grid_2d::Coord;

use crate::{game::Direction, player::TeamColor};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PieceOrientation {
    Horizontal,
    Vertical,
    DiagonalUp,
    DiagonalDown,
}

impl PieceOrientation {
    pub fn into_directions(&self) -> (Direction, Direction) {
        match self {
            PieceOrientation::Horizontal => (Direction::Right, Direction::Left),
            PieceOrientation::Vertical => (Direction::Top, Direction::Bottom),
            PieceOrientation::DiagonalUp => (Direction::TopRight, Direction::BottomLeft),
            PieceOrientation::DiagonalDown => (Direction::TopLeft, Direction::BottomRight),
        }
    }
}

pub static ALL_ORIENTATIONS: [PieceOrientation; 4] = [
    PieceOrientation::Horizontal, 
    PieceOrientation::Vertical, 
    PieceOrientation::DiagonalDown, 
    PieceOrientation::DiagonalUp
];

pub enum GameResult {
    Won {
        team_color: TeamColor,
        orientation: PieceOrientation,
        winning_coords: HashSet<Coord>
    },
    Stalemate
}