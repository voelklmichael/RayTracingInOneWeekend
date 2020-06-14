pub type Float = f32;

pub fn generate_random() -> Float {
    rand::prelude::random()
}
pub fn generate_random_in_between(min: Float, max: Float) -> Float {
    min + (max - min) * rand::prelude::random::<Float>()
}
#[test]
fn bla() {
    for _ in 0..100 {
        let r = generate_random();
        assert!(r >= 0.);
        assert!(r <= 1.);
    }
}
