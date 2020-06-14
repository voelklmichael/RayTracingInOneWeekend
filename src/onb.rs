use crate::vec3::Direction;
pub fn construct_onb_from_view_up(
    direction: &Direction,
    view_up: &Direction,
) -> (Direction, Direction) {
    let direction = direction.unit_vector();
    let view_up = view_up.unit_vector();

    let u = view_up.cross(&direction);
    let v = direction.cross(&u);

    (u, v)
}
