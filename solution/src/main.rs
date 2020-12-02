use std::io::{self, BufRead};
use std::collections::HashMap;
// use std::prelude::*;

fn main() -> io::Result<()> {
  // let mut input: Vec<i32> = Vec::new();
  let mut input: HashMap<i32, Result<(), ()>> = HashMap::new();

  let stdin = io::stdin();

  // println!("\nYou've enetered: ");
  for ref line in stdin.lock().lines() {
    if let Ok(s) = &line {
      if *s == "" { break; }
      // else { input.push( s.as_str().parse().unwrap() ) }
      else { input.insert( s.as_str().parse().unwrap(), Ok(()) ); }
    }
  }

  for x in input.keys() {
    if input.contains_key(&(2020 - x)) {
      println!("Answer: {}", x * (2020-x));
      return Ok(())
    }
  }

  Ok(())
}
