pub use crate::types::Float;

struct Vec3 {
    x: Float,
    y: Float,
    z: Float,
}
impl Vec3 {
    fn origin() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
    fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }
    fn x(&self) -> Float {
        self.x
    }
    fn y(&self) -> Float {
        self.y
    }
    fn z(&self) -> Float {
        self.z
    }
    fn extract(&self) -> (Float, Float, Float) {
        (self.x, self.y, self.z)
    }
    fn add(&self, other: Self) -> Self {
        let Self { x, y, z } = self;
        let Self {
            x: ox,
            y: oy,
            z: oz,
        } = other;
        Self {
            x: x + ox,
            y: y + oy,
            z: z + oz,
        }
    }
    fn substract(&self, other: Self) -> Self {
        let Self { x, y, z } = self;
        let Self {
            x: ox,
            y: oy,
            z: oz,
        } = other;
        Self {
            x: x - ox,
            y: y - oy,
            z: z - oz,
        }
    }
    fn scale(&self, factor: Float) -> Self {
        let Self { x, y, z } = self;
        Self {
            x: x * factor,
            y: y * factor,
            z: z * factor,
        }
    }
    fn l2_norm(&self) -> Float {
        let Self { x, y, z } = self;
        x * x + y * y + z * z
    }
}
pub struct Point {
    vec: Vec3,
}
impl Point {
    pub fn origin() -> Self {
        Self {
            vec: Vec3::origin(),
        }
    }
}
pub struct Direction {
    vec: Vec3,
}
impl Direction {
    pub fn new_z() -> Self {
        Self {
            vec: Vec3::new(0., 0., 1.),
        }
    }
    pub fn l2_norm(&self) -> Float {
        self.vec.l2_norm()
    }
    pub fn length(&self) -> Float {
        self.l2_norm().sqrt()
    }
}
impl std::ops::Add<Direction> for Point {
    type Output = Point;
    fn add(self, rhs: Direction) -> Self::Output {
        Point {
            vec: self.vec + rhs.vec,
        }
    }
}

impl std::ops::Sub<Direction> for Direction {
    type Output = Point;
    fn sub(self, rhs: Direction) -> Self::Output {
        Point {
            vec: self.vec - rhs.vec,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        self.add(rhs)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self.substract(rhs)
    }
}
impl std::ops::Mul<Float> for Vec3 {
    type Output = Vec3;
    fn mul(self, factor: Float) -> Self::Output {
        self.scale(factor)
    }
}

impl std::ops::Mul<Float> for Direction {
    type Output = Direction;
    fn mul(self, factor: Float) -> Self::Output {
        let Direction { vec } = self;
        Direction {
            vec: vec.scale(factor),
        }
    }
}
impl std::ops::Div<Float> for Direction {
    type Output = Direction;
    fn div(self, factor: Float) -> Self::Output {
        let Direction { vec } = self;
        Direction {
            vec: vec.scale(1. / factor),
        }
    }
}
