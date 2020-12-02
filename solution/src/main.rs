use text_io::scan;
use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();

  let mut valid_count: u16 = 0;

  let mut min: u16;
  let mut max: u16;
  let mut ch: char;
  let mut string: String;

  for line in stdin.lock().lines() {

    let l = line.expect("unexpected string");

    if l.is_empty() {
      break;
    }
    
    scan!(l.bytes() => "{}-{} {}: {}", min, max, ch, string);

    // let mut ctr: u16 = 0;
  
    // for c in string.chars() {
    //   if c == ch {
    //     ctr += 1;
    //   }
    // }
  
    // if ctr <= max && ctr >= min {
    //   valid_count += 1;
    // }

    // if (string.chars().nth((min-1) as usize).unwrap() == ch) ^ (string.chars().nth((max-1) as usize).unwrap() == ch) {
    //   valid_count += 1;
    // }

    if (string.as_bytes()[(min-1) as usize] == ch as u8) ^ (string.as_bytes()[(max-1) as usize] == ch as u8) {
      valid_count += 1;
    }

  }
  
  println!("Answer: {}", valid_count);
}
