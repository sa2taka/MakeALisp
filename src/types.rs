use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum MalValue {
  Int(i64),
  Symbol(String),
  List(Vec<MalValue>),
}

#[derive(Debug)]
pub struct MalError {
  pub text: String,
}

impl Display for MalError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "MalError: {}", self.text)
  }
}

impl Error for MalError {}

pub type MalResult = Result<MalValue, MalError>;
