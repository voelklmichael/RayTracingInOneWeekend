pub use crate::vec3::{Direction, Float, Point};

pub struct Ray {
    position: Point,
    direction: Direction,
}

impl Ray {
    pub fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
    pub fn position(&self) -> &Point {
        &self.position
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn progress(&mut self, t: Float) {
        self.position = self.position.clone() + (self.direction.clone() * t)
    }
    pub fn next(&self, t: Float) -> Point {
        self.position.clone() + (self.direction.clone() * t)
    }
}
