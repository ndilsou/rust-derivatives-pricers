use crate::payoffs::Payoff;

///A Vanilla Option product
pub struct VanillaOption {
    payoff: Box<dyn Payoff>,
    expiry: f64,
}

impl VanillaOption {
    pub fn new(payoff: Box<dyn Payoff>, expiry: f64) -> VanillaOption {
        VanillaOption { payoff, expiry }
    }

    pub fn expiry(&self) -> f64 {
        self.expiry
    }

    pub fn flow(&self, spot: f64) -> f64 {
        self.payoff.apply(spot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payoffs::CallPayoff;

    #[test]
    fn can_create_vanilla_option() {
        let payoff = Box::new(CallPayoff::new(100.0));
        let option = VanillaOption::new(payoff, 100.0);

        assert_eq!(option.expiry(), 100.0);
    }
}
