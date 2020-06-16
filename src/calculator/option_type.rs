pub enum OptionType {
  Call,
  Put
}

impl Copy for OptionType {}

impl Clone for OptionType {
  fn clone(&self) -> Self {
    *self
  }
}