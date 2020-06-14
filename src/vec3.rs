pub use crate::types::Float;

#[derive(Debug, Clone, PartialEq)]
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

    fn scale(&self, factor: Float) -> Self {
        let Self { x, y, z } = self;
        Self {
            x: x * factor,
            y: y * factor,
            z: z * factor,
        }
    }
    fn l2_norm_squared(&self) -> Float {
        let Self { x, y, z } = self;
        x * x + y * y + z * z
    }
    fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross(&self, other: &Self) -> Self {
        let Self { x, y, z } = self;
        let Self {
            x: ox,
            y: oy,
            z: oz,
        } = other;
        Self {
            x: y * oz - z * oy,
            y: z * ox - x * oz,
            z: x * oy - y * ox,
        }
    }
    fn invert(&mut self) {
        self.x *= -1.;
        self.y *= -1.;
        self.z *= -1.;
    }
    fn random() -> Self {
        use crate::types::generate_random;
        Self::new(generate_random(), generate_random(), generate_random())
    }
    fn random_in_between(min: Float, max: Float) -> Self {
        use crate::types::generate_random;
        let diff = max - min;
        let random = || min + diff * generate_random();
        Self::new(random(), random(), random())
    }
    fn random_unit_vector() -> Self {
        let r = Self::random();
        let l = r.l2_norm_squared();
        r * (1. / l)
    }
    pub fn lambertian_z() -> Self {
        use crate::types::generate_random_in_between;
        let a = generate_random_in_between(0., std::f64::consts::PI as Float * 2.);
        let z = generate_random_in_between(-1., 1.);
        let r = (1. - z * z).sqrt();
        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Point {
    vec: Vec3,
}
impl Point {
    pub fn origin() -> Self {
        Self {
            vec: Vec3::origin(),
        }
    }
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self {
            vec: Vec3::new(x, y, z),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Direction {
    vec: Vec3,
}
impl Direction {
    pub fn new_x() -> Self {
        Self {
            vec: Vec3::new(1., 0., 0.),
        }
    }
    pub fn new_y() -> Self {
        Self {
            vec: Vec3::new(0., 1., 0.),
        }
    }
    pub fn new_z() -> Self {
        Self {
            vec: Vec3::new(0., 0., 1.),
        }
    }
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self {
            vec: Vec3::new(x, y, z),
        }
    }
    pub fn x(&self) -> Float {
        self.vec.x
    }
    pub fn y(&self) -> Float {
        self.vec.y
    }
    pub fn z(&self) -> Float {
        self.vec.z
    }
    pub fn extract(&self) -> (Float, Float, Float) {
        (self.vec.x, self.vec.y, self.vec.z)
    }
    pub fn l2_norm_squared(&self) -> Float {
        self.vec.l2_norm_squared()
    }
    pub fn length(&self) -> Float {
        self.l2_norm_squared().sqrt()
    }
    pub fn make_unit_vector(&mut self) {
        let l = self.length();
        self.vec.x /= l;
        self.vec.y /= l;
        self.vec.z /= l;
    }
    pub fn unit_vector(&self) -> Self {
        let l = self.length();
        Self {
            vec: Vec3 {
                x: self.vec.x / l,
                y: self.vec.y / l,
                z: self.vec.z / l,
            },
        }
    }
    pub fn dot(&self, other: &Self) -> Float {
        self.vec.dot(&other.vec)
    }
    pub fn invert(&mut self) {
        self.vec.invert();
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            vec: self.vec.cross(&other.vec),
        }
    }
    pub fn random_in_unit_sphere() -> Self {
        let vec = Vec3::random_unit_vector();
        let vec = vec * crate::types::generate_random();
        Self { vec }
    }
    pub fn random_unit_vector() -> Self {
        Self {
            vec: Vec3::random_unit_vector(),
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            vec: self.vec.clone() + other.vec.clone(),
        }
    }
    pub fn lambertian_z() -> Self {
        Self {
            vec: Vec3::lambertian_z(),
        }
    }
    pub fn reflect(&self, normal: &Self) -> Self {
        let dot = self.dot(normal);
        self.add(&(normal * (dot * -2.)))
    }
    pub fn random_lambertian(normal: &Self) -> Self {
        normal.add(&Self {
            vec: Vec3::random_unit_vector(),
        })
    }
    pub fn refract(uv: &Self, n: &Self, etai_over_etat: Float) -> Self {
        let cos_theta = n.dot(&(uv * (-1.)));
        let r_out_parallel = (uv.add(&(n * cos_theta))) * etai_over_etat;

        let r_out_perp = n * (-(1.0 - r_out_parallel.l2_norm_squared()).sqrt());
        r_out_parallel.add(&r_out_perp)
    }
    pub fn random_in_unit_disk() -> Self {
        loop {
            use crate::types::generate_random_in_between;
            let vec = Vec3::new(
                generate_random_in_between(-1., 1.),
                generate_random_in_between(-1., 1.),
                0.,
            );
            if vec.l2_norm_squared() < 1. {
                return Self { vec };
            }
        }
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

impl std::ops::Sub<Point> for Point {
    type Output = Direction;
    fn sub(self, rhs: Point) -> Self::Output {
        Direction {
            vec: self.vec - rhs.vec,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        let Self { x, y, z } = self;
        let Self {
            x: ox,
            y: oy,
            z: oz,
        } = rhs;
        Self {
            x: x + ox,
            y: y + oy,
            z: z + oz,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        let Self { x, y, z } = self;
        let Self {
            x: ox,
            y: oy,
            z: oz,
        } = rhs;
        Self {
            x: x - ox,
            y: y - oy,
            z: z - oz,
        }
    }
}

impl std::ops::Mul<Float> for &Direction {
    type Output = Direction;
    fn mul(self, factor: Float) -> Self::Output {
        Direction {
            vec: self.vec.scale(factor),
        }
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

#[test]
fn cross_product_test() {
    let dx = Direction::new_x();
    let dy = Direction::new_y();
    let dz = Direction::new_z();
    assert_eq!(&dz, &dx.cross(&dy));
    assert_eq!(&dx, &dy.cross(&dz));
    assert_eq!(&dy, &dz.cross(&dx));

    assert_eq!(&(dz.clone() * -1.), &dy.cross(&dx));
    assert_eq!(&(dx.clone() * -1.), &dz.cross(&dy));
    assert_eq!(&(dy.clone() * -1.), &dx.cross(&dz));
}
#[test]
fn dot_product_test() {
    let dx = Direction::new_x();
    let dy = Direction::new_y();
    let dz = Direction::new_z();
    assert_eq!(0., dx.dot(&dy));
    assert_eq!(0., dy.dot(&dz));
    assert_eq!(0., dz.dot(&dx));

    assert_eq!(0., dy.dot(&dx));
    assert_eq!(0., dz.dot(&dy));
    assert_eq!(0., dx.dot(&dz));

    assert_eq!(1., dx.dot(&dx));
    assert_eq!(1., dy.dot(&dy));
    assert_eq!(1., dz.dot(&dz));
}
#[test]
fn failure_test_to_check_github() {
    assert!(false, "Failure test to check github")
}
