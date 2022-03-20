use crate::types as MalTypes;
use crate::types::MalValue;

impl MalValue {
  pub fn pr_str(&self) -> String {
    match self {
      MalValue::Int(i) => format!("{}", i),
      MalValue::Symbol(s) => s.clone(),
      MalValue::List(l) => format!(
        "({})",
        l.iter()
          .map(|x| x.pr_str())
          .collect::<Vec<String>>()
          .join(" ")
      ),
    }
  }
}

pub fn pr_str(value: MalTypes::MalValue) -> String {
  return value.pr_str();
}
