use std::io::{self, BufRead};
// use text_io::scan;

fn main() {
  let stdin = io::stdin();
  let mut map: Vec<Vec<char>> = Vec::new();

  for res in stdin.lock().lines() {
    let line = res.expect("unexpected string");

    if line.is_empty() {
      break;
    }
    
    let mut chars: Vec<char> = vec![];
    for ch in line.chars() {
      chars.push(ch);
    }
    map.push(chars);
  }

  // let mut ctr = 0;
  // for v1 in map.iter() {
  //   print!("{}: ", ctr);
  //   ctr += 1;
  //   for v2 in v1.iter() {
  //     print!("{}", v2);
  //   }
  //   println!("\n");
  // }

  let mut pos: (usize, usize) = (0, 0);
  let mut trees = 0;

  let max_x = map[0].len();
  let max_y = map.len();

  // println!("height: {}, width: {}", max_x, max_y);
  // println!("{:?}", map);

  loop {
    pos.0 += 3;
    pos.1 += 1;
    if pos.0 >= max_x {
      pos.0 = pos.0 - max_x;
    }
    if pos.1 >= max_y {
      break;
    }
    // println!("x: {}, y: {}", pos.0, pos.1);
    if map[pos.1][pos.0] == '#' {
      trees += 1;
    }
  }

  println!("Answer: {}", trees);
}
