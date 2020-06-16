use crate::calculator::market_variables::MarketVariables;
use crate::calculator::pricing;
use crate::calculator::option_type::OptionType;
use statrs::distribution::{Univariate, Normal};
use rand::distributions::Distribution;

pub struct European {
  option_type: OptionType,
  k: f64,
  t: f64,
}

impl European {
  pub fn new(option_type: OptionType, k: f64, t: f64) -> European {
    Self { option_type, k, t }
  }

  pub fn payoff(option_type: OptionType, s: f64, k: f64) -> f64 {
    match option_type {
      OptionType::Call => (s - k).max(0.0),
      OptionType::Put => (k - s).max(0.0)
    }
  }
}

impl pricing::Payoff for European {
  fn get_payoff(&self, mv_ref: &MarketVariables) -> f64 {
    European::payoff(self.option_type, mv_ref.s, self.k)
  }
}

impl pricing::BinomialTree for European {
  fn node_price(s: f64, u: f64, d: f64, n_down: usize, n: usize) -> f64 {
    s * u.powf( (n - n_down) as f64 ) * d.powf(n_down as f64)
  }

  fn binomial_tree_theo(&self, mv_ref: &MarketVariables, nstep: usize) -> f64 {
    let last_layer_size: usize = nstep + 1;
    let s: f64 = mv_ref.s;
    let q: f64 = mv_ref.q;
    let r: f64 = mv_ref.r;
    let vol: f64 = mv_ref.vol;
    let t: f64 = self.t;
    let k: f64 = self.k;
    let dt: f64 = t / (nstep as f64);
    let u: f64 = (vol * dt.sqrt()).exp();
    let d: f64 = 1.0 / u;
    let prob: f64 = (((r - q) * dt).exp() - d) / (u - d);
    let df: f64 = (-r * dt).exp();

    let mut layer: Vec<f64> = (0..last_layer_size).map(|down| {
        let s_t = European::node_price(s, u, d, down, nstep);
        European::payoff(self.option_type, s_t, k)
      }).collect();
   
    for i in (0..nstep).rev() {
      for j in 0..i+1 {
        layer[j] = df * (layer[j] * prob + layer[j + 1] * (1.0 - prob));
      }
    }

    return layer[0]
  }
}

impl pricing::Analytic for European {
  fn analytic_theo(&self, mv_ref: &MarketVariables) -> f64 {
    let s: f64 = mv_ref.s;
    let q: f64 = mv_ref.q;
    let r: f64 = mv_ref.r;
    let vol: f64 = mv_ref.vol;
    let t: f64 = self.t;
    let k: f64 = self.k;
    let d1: f64 = ((s / k).ln() + (r - q + 0.5 * vol * vol) * t) / (vol * t.sqrt());
    let d2: f64 = d1 - vol * t.sqrt();
    let sign: f64 = match self.option_type {
      OptionType::Call => 1.0,
      OptionType::Put => -1.0
    };

    // returns error if mean or std = nan or std < 0 
    let normal: Normal = Normal::new(0.0, 1.0).unwrap();

    let first_term: f64 = s * (-q * t).exp() * normal.cdf(sign * d1);
    let second_term: f64 = k * (-r * t).exp() * normal.cdf(sign * d2);
    sign * (first_term - second_term)
  }
}

impl pricing::MonteCarlo for European {
  fn simulate_price(&self, mv_ref: &MarketVariables) -> f64 {
    let s: f64 = mv_ref.s;
    let q: f64 = mv_ref.q;
    let r: f64 = mv_ref.r;
    let vol: f64 = mv_ref.vol;
    let t: f64 = self.t;
    // returns error if mean or std = nan or std < 0 
    let normal = Normal::new(0.0, 1.0).unwrap();

    let mut rng = rand::thread_rng();
    let e: f64 = normal.sample(&mut rng);
    let expectation: f64 = s * ((r - q - 0.5 * vol * vol) * t).exp();
    let diffusion: f64 = vol * t.sqrt();

    expectation * (diffusion * e).exp()
  }

  fn montecarlo_theo(&self, mv_ref: &MarketVariables, n: usize) -> f64 {
    let option_type: OptionType = self.option_type;
    let k: f64 = self.k;
    let mut sum_payoff: f64 = 0.0;
    for _ in 0..n {
      let s: f64 = self.simulate_price(mv_ref);
      sum_payoff += European::payoff(option_type, s, k);
    }
    sum_payoff / (n as f64)
  }
}