use rand::prelude::*;

pub fn get_one_gaussion_by_box_muller() -> f64 {
    let mut x: f64;
    let mut y: f64;

    let mut rng = thread_rng();
    let mut size_squared: f64;
    loop {
        x = 2.0 * rng.gen::<f64>() - 1.0;
        y = 2.0 * rng.gen::<f64>() - 1.0;
        size_squared = x * x + y * y;
        if size_squared < 1.0 {
            break;
        }
    }

    x * (-2.0 * size_squared.ln() / size_squared).sqrt()
}

pub fn get_one_gaussian_by_summation() -> f64 {
    let mut rng = thread_rng();
    let summation: f64 = (0..12).map(|_| rng.gen::<f64>()).sum();

    summation - 6.0
}
