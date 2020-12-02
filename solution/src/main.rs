use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
  let mut input: HashMap<i32, Result<(), ()>> = HashMap::new();

  let stdin = io::stdin();

  for ref line in stdin.lock().lines() {
    if let Ok(s) = &line {
      if *s == "" {
        break;
      }
      else {
        input.insert(s.as_str().parse().unwrap(), Ok(()));
      }
    }
  }

  for x in input.keys() {
    for y in input.keys() {
      if input.contains_key(&(2020 - x - y)) {
        println!("Answer: {}", x * y * (2020 - x - y));
        return Ok(());
      }
    }
  }

  Ok(())
}
