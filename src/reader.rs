use crate::types as MalTypes;
use regex::Regex;

struct Reader {
  tokens: Vec<String>,
  position: usize,
}

impl Reader {
  fn next(&mut self) -> Option<String> {
    self.position += 1;
    Some(self.tokens.get(self.position - 1)?.to_string())
  }

  fn peek(&self) -> Option<String> {
    Some(self.tokens.get(self.position)?.to_string())
  }
}

fn tokenize(str: &String) -> Vec<String> {
  let regexp =
    Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();

  return regexp
    .captures_iter(str)
    .map(|matches| {
      if matches[1].starts_with(";") {
        return String::from("");
      }
      String::from(&matches[1])
    })
    .collect();
}

fn get_pair_token(token: &String) -> String {
  (match &token[..] {
    "(" => ")",
    "[" => "]",
    "{" => "}",
    _ => "",
  })
  .to_string()
}

fn read_atom(reader: &mut Reader) -> MalTypes::MalResult {
  let integer_regexp = Regex::new(r#"^-?[0-9]+$"#).unwrap();

  let token = reader.next().ok_or(MalTypes::MalError {
    text: format!("unexpected"),
  })?;

  if integer_regexp.is_match(&token) {
    Ok(MalTypes::MalValue::Int(token.parse().unwrap()))
  } else {
    Ok(MalTypes::MalValue::Symbol(token))
  }
}

fn read_list(reader: &mut Reader) -> MalTypes::MalResult {
  let mut list_tokens: Vec<MalTypes::MalValue> = vec![];
  let left_token = reader.next().ok_or(MalTypes::MalError {
    text: "syntax error".to_string(),
  })?;
  let right_token = get_pair_token(&left_token);
  loop {
    let token = reader.peek().ok_or(MalTypes::MalError {
      text: format!("syntax error; expected '{}', but EOR appeared", right_token),
    })?;
    if token == right_token {
      break;
    }
    list_tokens.push(read_form(reader)?);
  }
  let _ = reader.next();
  match &right_token[..] {
    ")" => Ok(MalTypes::MalValue::List(list_tokens)),
    _ => Err(MalTypes::MalError {
      text: format!("syntax error; expected '{}'", right_token),
    }),
  }
}

fn read_form(reader: &mut Reader) -> MalTypes::MalResult {
  let token = reader.peek().ok_or(MalTypes::MalError {
    text: "unexpected".to_string(),
  })?;

  match &token[..] {
    "(" => read_list(reader),
    _ => read_atom(reader),
  }
}

pub fn read_str(str: &String) -> MalTypes::MalResult {
  let tokens = tokenize(str);
  let mut reader = Reader {
    tokens: tokens,
    position: 0,
  };
  read_form(&mut reader)
}
