use std::error::Error;
use std::fmt;
use std::fmt::Display;

pub type Args = Vec<MalValue>;
pub type Func = fn(Args) -> MalResult;

#[derive(Debug, Clone)]
pub enum MalValue {
  Int(i64),
  Symbol(String),
  List(Vec<MalValue>),
  Func(Func),
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

impl MalValue {
  pub fn apply(&self, args: Args) -> MalResult {
    match *self {
      MalValue::Func(f) => f(args),
      _ => Err(MalError {
        text: "do not call".to_string(),
      }),
    }
  }
}
