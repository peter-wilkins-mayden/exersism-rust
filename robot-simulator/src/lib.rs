// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

struct Position {
    x: i32,
    y: i32,
}

pub struct Robot {
    direction: Direction,
    position: Position,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            direction: d,
            position: Position { x, y },
        }
    }

    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => Robot::new(self.position.x, self.position.y, Direction::East),
            Direction::East => Robot::new(self.position.x, self.position.y, Direction::South),
            Direction::South => Robot::new(self.position.x, self.position.y, Direction::West),
            Direction::West => Robot::new(self.position.x, self.position.y, Direction::North),
        }
    }

    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => Robot::new(self.position.x, self.position.y, Direction::West),
            Direction::West => Robot::new(self.position.x, self.position.y, Direction::South),
            Direction::South => Robot::new(self.position.x, self.position.y, Direction::East),
            Direction::East => Robot::new(self.position.x, self.position.y, Direction::North),
        }
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Robot::new(self.position.x, self.position.y + 1, Direction::North),
            Direction::West => Robot::new(self.position.x - 1, self.position.y, Direction::West),
            Direction::South => Robot::new(self.position.x, self.position.y - 1, Direction::South),
            Direction::East => Robot::new(self.position.x + 1, self.position.y, Direction::East),
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars()
            .fold(self, |robot, c| {
                match c {
                    'R' => robot.turn_right(),
                    'L' => robot.turn_left(),
                    'A' => robot.advance(),
                    _ => panic!("unrecognised instruction")
                }
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
