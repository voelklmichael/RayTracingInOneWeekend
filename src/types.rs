pub type Float = f32;

pub fn generate_random() -> Float {
    rand::prelude::random()
}
/*
pub fn clamp(x: Float, min: Float, max: Float) -> Float {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}*/

#[test]
fn bla() {
    for _ in 0..100 {
        let r = generate_random();
        assert!(r >= 0.);
        assert!(r <= 1.);
    }
}
