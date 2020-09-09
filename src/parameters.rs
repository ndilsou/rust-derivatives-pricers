pub trait Parameter {
    fn integral(&self, t1: f64, t2: f64) -> f64;

    fn integral_square(&self, t1: f64, t2: f64) -> f64;

    fn mean(&self, t1: f64, t2: f64) -> f64;

    fn root_mean_square(&self, t1: f64, t2: f64) -> f64;
}

pub struct ConstantParameter {
    constant: f64,
    constant_squared: f64
}

impl ConstantParameter {
    pub fn new(constant: f64) -> ConstantParameter {
        ConstantParameter { constant, constant_squared: constant * constant }
    }
}
impl Parameter for ConstantParameter {

    fn integral(&self, t1: f64, t2: f64) -> f64 {
        self.constant * (t2 - t1)
    }

    fn integral_square(&self, t1: f64, t2: f64) -> f64 {
        self.constant_squared * (t2 - t1)
    }

    fn mean(&self, t1: f64, t2: f64) -> f64 {
        let total = self.integral(t1, t2);
        total / (t2 - t1)
    }

    fn root_mean_square(&self, t1: f64, t2: f64) -> f64 {
        let total = self.integral_square(t1, t2);
        total / (t2 - t1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_parameter_methods() {
        let param = ConstantParameter::new(10.0);
        assert_eq!(param.integral(0.0, 2.0), 20.0);
        assert_eq!(param.integral_square(0.0, 2.0), 200.0);
        assert_eq!(param.mean(0.0, 2.0), 10.0);
        assert_eq!(param.root_mean_square(0.0, 2.0), 100.0);
    }

}
