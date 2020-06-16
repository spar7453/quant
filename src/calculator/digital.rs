use crate::calculator::market_variables::MarketVariables;
use crate::calculator::market_variables as mv;
use crate::calculator::pricing;
use crate::calculator::option_type::OptionType;

pub struct Digital {
  option_type: OptionType,
  k: f64,
  t: f64,
  mv: mv::MarketVariables
}

impl Digital {
  pub fn new(option_type: OptionType, k: f64, t: f64, mv: mv::MarketVariables) -> Digital {
    Self { option_type: option_type, k: k, t: t, mv: mv }
  }
}

impl pricing::Payoff for Digital {
  fn get_payoff(&self, mv_ref: &MarketVariables) -> f64 {
    1.0
  }
}