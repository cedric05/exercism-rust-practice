#[derive(Debug)]
pub struct ChessPosition {
    x: i8,
    y: i8,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank >= 8 || file >= 8 {
            None
        } else {
            Some(ChessPosition {
                x: rank as i8,
                y: file as i8,
            })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.x == other.pos.x
            || self.pos.y == other.pos.y
            || (self.pos.x - other.pos.x).abs() == (self.pos.y - other.pos.y).abs()
    }
}
