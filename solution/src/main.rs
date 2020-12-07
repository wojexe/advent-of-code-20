fn main() {
  let input = include_str!("../input.in");
  
  let mut matrix = [[false; 8]; 300];

  for line in input.lines() {
    let mut r_start: u8 = 0;
    let mut r_end: u8 = 127;
    let mut c_start: u8 = 0;
    let mut c_end: u8 = 7;
    for ch in line.chars() {
      match ch {
        'F' => { r_end = r_start + (r_end-r_start)/2 },
        'B' => { r_start = r_start + (r_end-r_start)/2 + 1 },
        'L' => { c_end = c_start + (c_end-c_start)/2 },
        'R' => { c_start = c_start + (c_end-c_start)/2 + 1 },
        _ => {}
      }
    }
    matrix[(r_start-1) as usize][c_start as usize] = true;
  }

  for (i, x) in matrix.iter().skip(1).enumerate() {
    for (j, y) in x.iter().enumerate() {
      if !y {
        println!("Answer: {}", (i+2) * 8 + j);
        return
      }
    }
  }

}
