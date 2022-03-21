use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

mod printers;
mod reader;
mod types;

use crate::types as MalTypes;
use crate::types::MalValue;

type Env = HashMap<String, MalTypes::Func>;

fn get_env() -> Env {
  let mut env: Env = HashMap::new();

  env.insert("+".to_string(), |args: MalTypes::Args| {
    handle_int_operation(|left, right| left + right, args)
  });
  env.insert("-".to_string(), |args: MalTypes::Args| {
    handle_int_operation(|left, right| left - right, args)
  });
  env.insert("*".to_string(), |args: MalTypes::Args| {
    handle_int_operation(|left, right| left * right, args)
  });
  env.insert("/".to_string(), |args: MalTypes::Args| {
    handle_int_operation(|left, right| left / right, args)
  });

  env
}

fn read(input: String) -> MalTypes::MalResult {
  reader::read_str(&input)
}

fn handle_int_operation(op: fn(i64, i64) -> i64, args: MalTypes::Args) -> MalTypes::MalResult {
  match (args[0].clone(), args[1].clone()) {
    (MalValue::Int(arg1), MalValue::Int(arg2)) => Ok(MalValue::Int(op(arg1, arg2))),
    _ => Err(MalTypes::MalError {
      text: "invalid args".to_string(),
    }),
  }
}

fn eval_ast(ast: MalTypes::MalValue, env: &Env) -> MalTypes::MalResult {
  match ast {
    MalValue::Symbol(s) => Ok(MalValue::Func(
      env
        .get(&s)
        .ok_or(MalTypes::MalError {
          text: "unknown symbol".to_string(),
        })?
        .clone(),
    )),
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

fn eval(input: MalTypes::MalResult, env: &Env) -> MalTypes::MalResult {
  let ast = input?.clone();
  match ast {
    MalValue::List(ref l) => {
      if l.len() == 0 {
        return Ok(ast);
      }
      match eval_ast(ast, env)? {
        MalValue::List(leaves) => {
          let func = leaves[0].clone();
          func.apply(leaves[1..].to_vec())
        }
        _ => Err(MalTypes::MalError {
          text: "unexpected".to_string(),
        }),
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

fn rep(input: String, env: &Env) -> String {
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
