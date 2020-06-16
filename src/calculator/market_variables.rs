#[allow(dead_code)]
#[derive(Debug)]
pub struct MarketVariables {
  pub s: f64,
  pub q: f64,
  pub r: f64,
  pub vol: f64
}

#[allow(dead_code)]
impl MarketVariables {
  pub fn new(s: f64, q: f64, r: f64, vol: f64) -> Self {
    Self { s, q, r, vol }
  }
  pub fn set_all(&mut self, s: f64, q: f64, r: f64, vol: f64) {
    self.s = s;
    self.q = q;
    self.r = r;
    self.vol = vol;
  }
  pub fn set_s(&mut self, s: f64) { self.s = s }
  pub fn set_q(&mut self, q: f64) { self.q = q }
  pub fn set_r(&mut self, r: f64) { self.r = r }
  pub fn set_vol(&mut self, vol: f64) { self.vol = vol }
}

impl Copy for MarketVariables {}

impl Clone for MarketVariables {
  fn clone(&self) -> Self {
    *self
  }
}