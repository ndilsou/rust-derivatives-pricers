///Defines a payoff applied to the underlying spot value.
pub trait Payoff {
    ///Applies the payoff to the spot value of the underlying.
    fn apply(&self, spot: f64) -> f64;
}

///Defines the payoff for a Call Option
#[derive(Debug)]
pub struct CallPayoff {
    strike: f64,
}

impl CallPayoff {
    pub fn new(strike: f64) -> CallPayoff {
        CallPayoff { strike }
    }
}

impl Payoff for CallPayoff {
    fn apply(&self, spot: f64) -> f64 {
        let payoff = spot - self.strike;
        if payoff > 0.0 {
            payoff
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_payoff_ok() {
        let etol = 1e-10;
        let payoff = CallPayoff::new(50.0_f64);

        let expected = 50.0_f64;
        let actual = payoff.apply(100.0_f64);

        assert!(
            (actual - expected).abs() < etol,
            "actual: {}, expected: {}",
            actual,
            expected
        );
    }
}
