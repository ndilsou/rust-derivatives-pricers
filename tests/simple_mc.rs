extern crate derivatives_pricer;
use derivatives_pricer::stats_collectors::MonteCarloStatsCollector;
use derivatives_pricer::parameters::Parameter;
use derivatives_pricer::payoffs::Payoff;
use derivatives_pricer::products::VanillaOption;
use derivatives_pricer::random::get_one_gaussion_by_box_muller;

pub fn simple_monte_carlo_1(
    expiry: f64,
    strike: f64,
    spot: f64,
    vol: f64,
    rate: f64,
    number_of_paths: u32,
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
        this_payoff = if this_payoff > 0.0 { this_payoff } else { 0.0 };
        running_sum += this_payoff;
    }

    let mut mean = running_sum / number_of_paths as f64;
    mean *= (-rate * expiry).exp();

    mean
}

pub fn simple_monte_carlo_2(
    payoff: &dyn Payoff,
    expiry: f64,
    spot: f64,
    vol: f64,
    rate: f64,
    number_of_paths: usize,
) -> f64 {
    let variance = vol * vol * expiry;
    let root_variance = variance.sqrt();
    let ito_correction = -0.5 * variance;

    let moved_spot = spot * (rate * expiry + ito_correction).exp();
    let mut running_sum = 0.0;
    let mut this_spot: f64;
    let mut this_gaussian: f64;
    let mut this_payoff: f64;
    for _ in 0..number_of_paths {
        this_gaussian = get_one_gaussion_by_box_muller();
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        this_payoff = payoff.apply(this_spot);
        running_sum += this_payoff;
    }

    let mut mean = running_sum / number_of_paths as f64;
    mean *= (-rate * expiry).exp();

    mean
}

pub fn simple_monte_carlo_3(
    product: VanillaOption,
    spot: f64,
    vol: &dyn Parameter,
    rate: &dyn Parameter,
    number_of_paths: u32,
) -> f64 {
    let expiry = product.expiry();
    let variance = vol.integral_square(0.0, expiry);
    let root_variance = variance.sqrt();
    let ito_correction = -0.5 * variance;

    let moved_spot = spot * (rate.integral(0.0, expiry) + ito_correction).exp();
    let mut running_sum = 0.0;
    let mut this_spot: f64;
    let mut this_gaussian: f64;
    let mut this_payoff: f64;
    for _ in 0..number_of_paths {
        this_gaussian = get_one_gaussion_by_box_muller();
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        this_payoff = product.flow(this_spot);
        running_sum += this_payoff;
    }

    let mut mean = running_sum / number_of_paths as f64;
    mean *= (-rate.integral(0.0, expiry)).exp();

    mean
}

pub fn simple_monte_carlo_4(
    product: VanillaOption,
    spot: f64,
    vol: &dyn Parameter,
    rate: &dyn Parameter,
    number_of_paths: u32,
    gatherer: &mut dyn MonteCarloStatsCollector
) {
    let expiry = product.expiry();
    let variance = vol.integral_square(0.0, expiry);
    let root_variance = variance.sqrt();
    let ito_correction = -0.5 * variance;

    let moved_spot = spot * (rate.integral(0.0, expiry) + ito_correction).exp();
    let discount_factor = (-rate.integral(0.0, expiry)).exp();
    let mut running_sum = 0.0;
    let mut this_spot: f64;
    let mut this_gaussian: f64;
    let mut this_payoff: f64;
    for _ in 0..number_of_paths {
        this_gaussian = get_one_gaussion_by_box_muller();
        this_spot = moved_spot * (root_variance * this_gaussian).exp();
        this_payoff = product.flow(this_spot);
        gatherer.accumulate_result(discount_factor * this_payoff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use derivatives_pricer::parameters::ConstantParameter;
    use derivatives_pricer::payoffs::CallPayoff;
    use derivatives_pricer::stats_collectors::MonteCarloMeanCollector;

    #[test]
    fn price_european_call() {
        let etol = 0.1;
        let expiry = 1.0;
        let strike = 100.0;
        let spot = 100.0;
        let vol = 0.25;
        let rate = 0.05;
        let num_paths = 200_000;
        let expected_price = 12.34;
        let actual_price = simple_monte_carlo_1(expiry, strike, spot, vol, rate, num_paths);
        assert!(
            (actual_price - expected_price).abs() < etol,
            "actual: {}, expected: {}",
            actual_price,
            expected_price
        );
    }

    #[test]
    fn price_european_call_with_payoff() {
        let etol = 0.1;
        let expiry = 1.0;
        let payoff = CallPayoff::new(100.0);
        let spot = 100.0;
        let vol = 0.25;
        let rate = 0.05;
        let num_paths = 100_000;
        let expected_price = 12.34;
        let actual_price = simple_monte_carlo_2(&payoff, expiry, spot, vol, rate, num_paths);
        assert!(
            (actual_price - expected_price).abs() < etol,
            "actual: {}, expected: {}",
            actual_price,
            expected_price
        );
    }

    #[test]
    fn price_european_call_with_product_and_params() {
        let etol = 0.1;
        let expiry = 1.0;
        let payoff = Box::new(CallPayoff::new(100.0));
        let product = VanillaOption::new(payoff, expiry);
        let spot = 100.0;
        let vol = ConstantParameter::new(0.25);
        let rate = ConstantParameter::new(0.05);
        let num_paths = 100_000;
        let expected_price = 12.34;
        let actual_price = simple_monte_carlo_3(product, spot, &vol, &rate, num_paths);
        assert!(
            (actual_price - expected_price).abs() < etol,
            "actual: {}, expected: {}",
            actual_price,
            expected_price
        );
    }

    #[test]
    fn price_european_call_with_stats_gatherer() {
        let etol = 0.1;
        let expiry = 1.0;
        let payoff = Box::new(CallPayoff::new(100.0));
        let product = VanillaOption::new(payoff, expiry);
        let spot = 100.0;
        let vol = ConstantParameter::new(0.25);
        let rate = ConstantParameter::new(0.05);
        let num_paths = 100_000;
        let expected_price = 12.34;
        let mut gatherer = MonteCarloMeanCollector::new();
        simple_monte_carlo_4(product, spot, &vol, &rate, num_paths, &mut gatherer);
        let stats = gatherer.running_stats();
        let actual_price = stats[0][0];
        assert!(
            (actual_price - expected_price).abs() < etol,
            "actual: {}, expected: {}",
            actual_price,
            expected_price
        );
    }
}
