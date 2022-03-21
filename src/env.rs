use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types as MalTypes;
use crate::types::MalValue;

#[derive(Debug)]
pub struct EnvData {
  data: RefCell<HashMap<String, MalTypes::MalValue>>,
  pub outer: Option<Env>,
}

pub type Env = Rc<EnvData>;

pub fn new(outer: Option<Env>) -> Env {
  Rc::new(EnvData {
    data: RefCell::new(HashMap::new()),
    outer: outer,
  })
}

pub fn set(env: &Env, key: MalTypes::MalValue, value: MalTypes::MalValue) -> MalTypes::MalResult {
  match key {
    MalValue::Symbol(s) => {
      let _ = env.data.borrow_mut().insert(s, value.clone());
      Ok(value)
    }
    _ => Err(MalTypes::MalError {
      text: "cannot set as symbol".to_string(),
    }),
  }
}

pub fn find(env: &Env, key: &str) -> Option<Env> {
  if env.data.borrow().contains_key(key) {
    return Some(env.clone());
  }

  match env.outer.clone() {
    Some(env) => find(&env, key),
    _ => None,
  }
}

pub fn get(env: &Env, key: &MalTypes::MalValue) -> MalTypes::MalResult {
  match key {
    MalValue::Symbol(ref s) => Ok(
      find(env, s)
        .ok_or(MalTypes::MalError {
          text: format!("'{}' is not found", s),
        })?
        .data
        .borrow()
        .get(s)
        .ok_or(MalTypes::MalError {
          text: format!("'{}' is not found", s),
        })?
        .clone(),
    ),
    _ => Err(MalTypes::MalError {
      text: "unexpected".to_string(),
    }),
  }
}
