use crate::world::Pos;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
}

pub struct Agent {
    pub name: &'static str,
    pub team: Team,
    pub pos: Pos,
}

impl Agent {
    pub fn new(name: &'static str, team: Team, pos: Pos) -> Self {
        Self { name, team, pos }
    }

    pub fn glyph(&self) -> char {
        match self.team {
            Team::Player => '@',
            Team::Enemy => 'E',
        }
    }
}
