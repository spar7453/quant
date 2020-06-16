pub trait AnalyticGreeks {
  fn delta(&self) -> f64;
  fn gamma(&self) -> f64;
  fn theta(&self) -> f64;
  fn rho(&self) -> f64;
}

pub trait BinomilaTreeGreeks {
  fn delta(&self) -> f64;
  fn gamma(&self) -> f64;
  fn theta(&self) -> f64;
  fn rho(&self) -> f64;
}

pub trait MonteCarloGreeks {
  fn delta(&self) -> f64;
  fn gamma(&self) -> f64;
  fn theta(&self) -> f64;
  fn rho(&self) -> f64;
}