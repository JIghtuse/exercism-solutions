// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { x: x, y: y, direction: d }
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Robot::new(self.x, self.y, new_direction)
    }

    pub fn turn_left(self) -> Self {
        unimplemented!()
    }

    pub fn advance(self) -> Self {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn instructions(self, instructions: &str) -> Self {
        unimplemented!()
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
