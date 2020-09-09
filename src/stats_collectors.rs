pub trait MonteCarloStatsCollector {
    fn accumulate_result(&mut self, result: f64);
    fn running_stats(&self) -> Vec<Vec<f64>>;
}

pub struct MonteCarloMeanCollector {
    running_sum: f64,
    paths_done: usize
}

impl MonteCarloMeanCollector {
    pub fn new() -> MonteCarloMeanCollector {
        MonteCarloMeanCollector { running_sum: 0.0, paths_done: 0 }
    }
}

impl MonteCarloStatsCollector for MonteCarloMeanCollector {

    fn accumulate_result(&mut self, result: f64) {
        self.running_sum += result;
        self.paths_done += 1;
    }

    fn running_stats(&self) -> Vec<Vec<f64>> {
        let mut results = vec![vec![0.0; 1]; 1];
        results[0][0] = self.running_sum / self.paths_done as f64;
        results
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_collector() {
        let mut mc_mean = MonteCarloMeanCollector::new();
        mc_mean.accumulate_result(2.0);
        mc_mean.accumulate_result(2.0);
        mc_mean.accumulate_result(2.0);
        let stats = mc_mean.running_stats();
        assert_eq!(stats[0][0], 2.0);
    }
}
