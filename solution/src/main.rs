use text_io::scan;
use std::io::{self, BufRead};

/*
byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)
*/

fn main() {
    let stdin = io::stdin();
    let mut temp_str: String = String::new();
    let mut empty_lines: u8 = 0;
    let mut current_password_counter: u8 = 0;
    let mut valid_password_counter: u8 = 0;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if !line.is_empty() {
            empty_lines = 0;
            temp_str += line.as_str();
            temp_str += " ";
        } else {
            empty_lines += 1;
            if empty_lines >= 2 {
                break;
            }

            temp_str.split_whitespace().for_each(|x| {
                let k: String;
                let v: String;
                scan!(x.bytes() => "{}:{}", k, v);
                match k.as_str() {
                    "byr" => current_password_counter += 1,
                    "iyr" => current_password_counter += 1,
                    "eyr" => current_password_counter += 1,
                    "hgt" => current_password_counter += 1,
                    "hcl" => current_password_counter += 1,
                    "ecl" => current_password_counter += 1,
                    "pid" => current_password_counter += 1,
                    "cid" => (),
                    _ => ()
                }
            });
            
            if current_password_counter >= 7 {
                valid_password_counter += 1;
            }
            current_password_counter = 0;
            temp_str.clear();
        }
    }
    println!("Answer: {}", valid_password_counter);
}
