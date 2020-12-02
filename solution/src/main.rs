use text_io::scan;
use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();

  let mut valid_count: u16 = 0;

  let mut min: u16 = 0;
  let mut max: u16 = 0;
  let mut ch: char = '0';
  let mut string: String = String::new();

  for line in stdin.lock().lines() {

    let l = line.expect("unexpected string");

    if l.is_empty() {
      break;
    }
    
    scan!(l.bytes() => "{}-{} {}: {}", min, max, ch, string);

    let mut ctr: u16 = 0;
  
    for c in string.chars() {
      if c == ch {
        ctr += 1;
      }
    }
  
    if ctr <= max && ctr >= min {
      valid_count += 1;
    }
  }
  
  println!("Answer: {}", valid_count);
}
