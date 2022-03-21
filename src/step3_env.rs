use itertools::Itertools;
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod env;
mod printers;
mod reader;
mod types;

use crate::env as MalEnv;
use crate::types as MalTypes;
use crate::types::MalValue;

fn handle_int_operation(op: fn(i64, i64) -> i64, args: MalTypes::Args) -> MalTypes::MalResult {
  match (args[0].clone(), args[1].clone()) {
    (MalValue::Int(arg1), MalValue::Int(arg2)) => Ok(MalValue::Int(op(arg1, arg2))),
    _ => Err(MalTypes::MalError {
      text: "invalid args".to_string(),
    }),
  }
}

fn get_env() -> MalEnv::Env {
  let env = MalEnv::new(None);

  let _ = MalEnv::set(
    &env,
    MalValue::Symbol("+".to_string()),
    MalValue::Func(|args: MalTypes::Args| handle_int_operation(|left, right| left + right, args)),
  );
  let _ = MalEnv::set(
    &env,
    MalValue::Symbol("-".to_string()),
    MalValue::Func(|args: MalTypes::Args| handle_int_operation(|left, right| left - right, args)),
  );
  let _ = MalEnv::set(
    &env,
    MalValue::Symbol("*".to_string()),
    MalValue::Func(|args: MalTypes::Args| handle_int_operation(|left, right| left * right, args)),
  );
  let _ = MalEnv::set(
    &env,
    MalValue::Symbol("/".to_string()),
    MalValue::Func(|args: MalTypes::Args| handle_int_operation(|left, right| left / right, args)),
  );

  env
}

fn read(input: String) -> MalTypes::MalResult {
  reader::read_str(&input)
}

fn eval_ast(ast: MalTypes::MalValue, env: &MalEnv::Env) -> MalTypes::MalResult {
  match ast {
    MalValue::Symbol(s) => MalEnv::get(env, &MalValue::Symbol(s)),
    MalValue::List(l) => {
      let mut args: MalTypes::Args = vec![];
      for arg in l.iter() {
        args.push(eval(Ok(arg.clone()), env)?);
      }
      Ok(MalValue::List(args))
    }
    _ => Ok(ast.clone()),
  }
}

fn eval(input: MalTypes::MalResult, env: &MalEnv::Env) -> MalTypes::MalResult {
  let ast = input?.clone();
  match ast {
    MalValue::List(ref l) => {
      if l.len() == 0 {
        return Ok(ast);
      }
      let value = l[0].clone();
      match value {
        MalValue::Symbol(s) if s == "def!" => {
          MalEnv::set(env, l[1].clone(), eval(Ok(l[2].clone()), env)?)
        }
        MalValue::Symbol(s) if s == "let*" => {
          let new_env = MalEnv::new(Some(env.clone()));
          let (left, right) = (l[1].clone(), l[2].clone());

          match left {
            MalValue::List(binds) => {
              for (b, e) in binds.iter().tuples() {
                match b {
                  MalValue::Symbol(_) => {
                    let _ =
                      MalEnv::set(&new_env, b.clone(), eval(Ok(e.clone()), &new_env.clone())?);
                  }
                  _ => {
                    return Err(MalTypes::MalError {
                      text: "let* key requires symbol.".to_string(),
                    })
                  }
                }
              }
            }
            _ => {
              return Err(MalTypes::MalError {
                text: "let* requires binding lists.".to_string(),
              })
            }
          }
          eval(Ok(right), &new_env)
        }
        _ => match eval_ast(ast, env)? {
          MalValue::List(leaves) => leaves[0].apply(leaves[1..].to_vec()),
          _ => Err(MalTypes::MalError {
            text: "unexpected".to_string(),
          }),
        },
      }
    }
    _ => eval_ast(ast, &env),
  }
}

fn print(input: MalTypes::MalResult) -> String {
  match input {
    Ok(value) => printers::pr_str(value),
    Err(err) => format!("{}", err),
  }
}

fn rep(input: String, env: &MalEnv::Env) -> String {
  let read_result = read(input);
  let eval_result = eval(read_result, env);
  let print_result = print(eval_result);

  return print_result;
}

fn main() {
  let mut rl = Editor::<()>::new();
  if rl.load_history(".mal-history").is_err() {
    eprintln!("No previous history.");
  }
  let env = get_env();

  loop {
    let readline = rl.readline("user> ");
    match readline {
      Ok(line) => {
        rl.add_history_entry(&line);
        rl.save_history(".mal-history").unwrap();
        if line.len() > 0 {
          let result = rep(line, &env);
          println!("{}", result);
        }
      }
      Err(ReadlineError::Interrupted) => continue,
      Err(ReadlineError::Eof) => break,
      Err(err) => {
        println!("Error: {:?}", err);
        break;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let env = get_env();

    let line = "(def! a 6)".to_string();
    let result = rep(line, &env);
    assert_eq!(result, "6");

    let line = "(def! b (+ a 2))".to_string();
    let result = rep(line, &env);
    println!("{}", result);
    assert_eq!(result, "8");
  }
}
