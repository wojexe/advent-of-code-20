use std::collections::BinaryHeap;
fn main() {
  let input = include_str!("../input.in");
  
  let mut id_heap: BinaryHeap<u16> = BinaryHeap::new();

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
    // println!("{} = {} * 8 + {}", (r_start as u16) * 8 + (c_start as u16), r_start, c_start);
    id_heap.push((r_start as u16) * 8 + (c_start as u16));
  }

  println!("Answer: {}", id_heap.peek().unwrap());

}
