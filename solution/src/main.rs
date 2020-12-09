use std::collections::HashMap;

fn main() {
  // get input from file at compiletime
  let input = include_str!("../input.in");

  let mut v_m: Vec<HashMap<char, u32>> = Vec::with_capacity(100);
  v_m.push(HashMap::new());

  let mut i_m: usize = 0;
  let mut c_lines: u32 = 0;
  
  let mut res: u32 = 0;

  for line in input.lines() {
    if line.is_empty() {
      for (_, ans_count) in v_m[i_m].iter() {
        if *ans_count == c_lines {
          res += 1;
        }
      }

      i_m += 1;
      v_m.push(HashMap::new());
      c_lines = 0;
    } else {
      for ch in line.chars() {
        *v_m[i_m].entry(ch).or_insert(0) += 1;
      }
      c_lines += 1;
    }
  }

  for (_, ans_count) in v_m[i_m].iter() {
    if *ans_count == c_lines {
      res += 1;
    }
  }

  println!("Answer: {}", res);
}
