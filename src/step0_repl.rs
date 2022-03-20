use rustyline::error::ReadlineError;
use rustyline::Editor;

fn read(input: String) -> String {
  return input;
}

fn eval(input: String) -> String {
  return input;
}

fn print(input: String) -> String {
  return input;
}

fn rep(input: String) -> String {
  let read_result = read(input);
  let eval_result = eval(read_result);
  let print_result = print(eval_result);

  return print_result;
}

fn main() {
  let mut rl = Editor::<()>::new();
  if rl.load_history(".mal-history").is_err() {
    eprintln!("No previous history.");
  }

  loop {
    let readline = rl.readline("user> ");
    match readline {
      Ok(line) => {
        rl.add_history_entry(&line);
        rl.save_history(".mal-history").unwrap();
        if line.len() > 0 {
          let result = rep(line);
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
