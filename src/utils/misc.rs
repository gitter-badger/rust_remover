use serenity::utils::Colour;
use rand::{thread_rng, Rng};

pub fn random_color() -> Colour {
    let mut rng = thread_rng();
    let r: u8 = rng.gen_range(0, 255);
    let g: u8 = rng.gen_range(0, 255);
    let b: u8 = rng.gen_range(0, 255);
    Colour::from_rgb(r, g, b)
}