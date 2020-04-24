use super::random::get_one_gaussion_by_box_muller as get_one_gaussion_by_box_muller;

pub fn simple_monte_carlo_1(
        expiry: f64,
        strike: f64,
        spot: f64,
        vol: f64,
        rate: f64,
        number_of_paths: u32
    ) -> f64 {
    let variance = vol * vol * expiry;
    let root_variance = variance.sqrt();
    let ito_correction = -0.5 * variance;

    let moved_spot = spot * (rate * expiry + ito_correction).exp();
    let mut running_sum = 0.0;
    let mut this_spot: f64;
    for _ in 0..number_of_paths {
        let this_gaussian = get_one_gaussion_by_box_muller();
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        let mut this_payoff = this_spot - strike;
        this_payoff = if this_payoff > 0.0 {
            this_payoff
        } else {
            0.0
        };
        running_sum += this_payoff;
    }

    let mut mean = running_sum / number_of_paths as f64;
    mean *= (-rate * expiry).exp();

    mean
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn price_european_call() {
        let etol = 0.1;
        let expiry = 1.0;
        let strike = 100.0;
        let spot = 100.0;
        let vol = 0.25;
        let rate = 0.05;
        let num_paths = 100_000;
        let expected_price = 12.34;
        let actual_price = simple_monte_carlo_1(expiry, strike, spot, vol, rate, num_paths);
        assert!((actual_price - expected_price).abs() < etol, "actual: {}, expected: {}", actual_price, expected_price);
    }
}
