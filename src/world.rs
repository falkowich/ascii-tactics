#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub walls: Vec<Pos>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            walls: Vec::new(),
        }
    }

    pub fn set_walls(&mut self, pos: Pos) {
        self.walls.push(pos);
    }

    pub fn is_walls(&self, pos: Pos) -> bool {
        self.walls.contains(&pos)
    }
}
