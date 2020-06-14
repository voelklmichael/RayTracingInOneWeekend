use crate::vec3::Direction;

pub struct ONB {
    pub u: Direction,
    pub v: Direction,
    pub w: Direction,
}

pub fn construct_onb_from_view_up(direction: &Direction, view_up: &Direction) -> ONB {
    let w = direction.unit_vector();
    let view_up = view_up.unit_vector();

    let u = view_up.cross(&w);
    let v = w.cross(&u);

    ONB { u, v, w }
}
