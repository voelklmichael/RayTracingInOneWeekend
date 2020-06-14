pub use crate::rgb::RGB;
pub use crate::vec3::{Direction, Float, Point};

pub struct Ray {
    position: Point,
    direction: Direction,
    brightness: RGB,
}

impl Ray {
    pub fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            direction,
            brightness: RGB::white(),
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
    pub fn direction_mut(&mut self) -> &mut Direction {
        &mut self.direction
    }
    pub fn adjust_brightness(&mut self, albedo: &RGB) {
        self.brightness *= albedo;
    }
    pub fn brightness(&self) -> &RGB {
        &self.brightness
    }
}
