// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, direction: d }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            direction: (match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }),
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            direction: (match self.direction {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            }),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Robot {
                y: self.y + 1,
                ..self
            },
            Direction::East => Robot {
                x: self.x + 1,
                ..self
            },
            Direction::South => Robot {
                y: self.y - 1,
                ..self
            },
            Direction::West => Robot {
                x: self.x - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let clone = self;
        instructions.chars().fold(clone, |prev, c| match c {
            'R' => prev.turn_right(),
            'L' => prev.turn_left(),
            'A' => prev.advance(),
            _ => prev,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
