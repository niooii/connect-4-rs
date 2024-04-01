#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn translated(&self, shift_x: i32, shift_y: i32) -> Option<Self> {
        let new_x = self.x as i32 + shift_x;
        let new_y = self.y as i32 + shift_y;
        if new_x < 0 || new_y < 0 {
            return None;
        }
        Some(
            Self {
                x: new_x as usize,
                y: self.y as usize
            }
        )
    }
}

pub struct Grid<T> {
    w: usize,
    h: usize,
    cells: Vec<T>
}

impl<T> Grid<T> {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            cells: Vec::with_capacity(w * h)
        }
    }

    // y is from the top of the grid to bottom
    pub fn get(&self, coord: &Coord) -> Option<&T> {
        let idx = self.coord_to_idx(coord);
        self.cells.get(idx)
    }

    pub fn set(&mut self, coord: &Coord, item: T) {
        let idx = self.coord_to_idx(coord);
        self.cells[idx] = item;
    }

    fn coord_to_idx(&self, coord: &Coord) -> usize {
        coord.y * self.h + coord.x
    }
}