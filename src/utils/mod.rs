use serenity::utils::Colour;
use rand::{thread_rng, Rng};
use serenity::Result as SerenityResult;
use serenity::model::Message as SerenityMessage;
use rayon;

// Additional Utilitys
pub mod sharekvp;
pub mod linear_parse;


pub fn check_msg(result: SerenityResult<SerenityMessage>) {
    if let Err(why) = result {
        warn!("Error sending message: {:?}", why);
    }
}

// Utils Functions;
#[inline]
pub fn nanosecond_to_milisecond(ns: i64, sigdig: i32) -> f64 {
    if ns <= 0 {
        return 0.0;
    }
    let x = ns as f64 / 1000000.0;
    let sc = 10f64.powf(x.abs().log10().floor());
    sc * round_with_precision(x / sc, sigdig)
}

#[inline]
pub fn round_with_precision(num: f64, precision: i32) -> f64 {
    let power = 10f64.powi(precision);
    (num * power).round() / power
}

pub fn quick_sort<T:PartialOrd+Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| quick_sort(lo), || quick_sort(hi));
}

pub fn partition<T:PartialOrd+Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    #[cfg_attr(feature = "clippy", allow(needless_range_loop))]
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

pub fn random_color() -> Colour {
    let mut rng = thread_rng();
    let r: u8 = rng.gen_range(0, 255);
    let g: u8 = rng.gen_range(0, 255);
    let b: u8 = rng.gen_range(0, 255);
    Colour::from_rgb(r, g, b)
}


