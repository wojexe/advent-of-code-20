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

  /*
  Right 1, down 1.
  Right 3, down 1. (This is the slope you already checked.)
  Right 5, down 1.
  Right 7, down 1.
  Right 1, down 2.
  */

  let increments: Vec<(usize, usize)> = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];
  let mut answer: u64 = 1;

  #[allow(non_snake_case)]
  for (iX, iY) in increments {
    loop {
      pos.0 += iX;
      pos.1 += iY;
      if pos.0 >= max_x {
        pos.0 = pos.0 - max_x;
      }
      if pos.1 >= max_y {
        pos.0 = 1;
        pos.1 = 1;
        break;
      }
      if map[pos.1][pos.0] == '#' {
        trees += 1;
        // println!("x: {}, y: {}, trees: {}", pos.0, pos.1, trees);
      }
    }
    // println!("Trees: {}", trees);
    answer *= trees;
    trees = 0;
    pos.0 = 0;
    pos.1 = 0;
  }

  println!("Answer: {}", answer);
}
