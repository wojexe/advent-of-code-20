use std::collections::HashSet;

fn main() {
  // get input from file at compiletime
  let input = include_str!("../input.in");

  let mut v: Vec<HashSet<char>> = Vec::with_capacity(100);
  v.push(HashSet::new());

  let mut idx: usize = 0;

  for line in input.lines() {
    if line.is_empty() {
      idx += 1;
      v.push(HashSet::new());
    } else {
      for ch in line.chars() {
        v[idx].insert(ch);
      }
    }
  }

  let result = v.iter().fold(0, |acc, map| acc+map.len());

  println!("Answer: {}", result);
}
