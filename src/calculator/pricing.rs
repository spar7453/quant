use crate::calculator::market_variables::MarketVariables;
pub trait Payoff {
  fn get_payoff(&self, mv_ref: &MarketVariables) -> f64;
}

pub trait BinomialTree {
  fn node_price(s: f64, u: f64, d: f64, up: usize, n: usize) -> f64;
  fn binomial_tree_theo(&self, mv_ref: &MarketVariables, nstep: usize) -> f64;
}

pub trait Analytic {
  fn analytic_theo(&self, mv_ref: &MarketVariables) -> f64;
}

pub trait MonteCarlo {
  fn simulate_price(&self, mv_ref: &MarketVariables) -> f64;
  fn montecarlo_theo(&self, mv_ref: &MarketVariables, n: usize) -> f64;
}