use text_io::scan;
// use colour::*;
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
                let mut v: String;
                scan!(x.bytes() => "{}:{}", k, v);
                // print!("{}: ", k);
                // green!("{} \t", v);
                // let prev = current_password_counter;
                match k.as_str() {
                    "byr" => { if (1920..=2002 as u16).contains(&v.parse::<u16>().unwrap()) { current_password_counter += 1 }; },
                    "iyr" => { if (2010..=2020 as u16).contains(&v.parse::<u16>().unwrap()) { current_password_counter += 1 }; },
                    "eyr" => { if (2020..=2030 as u16).contains(&v.parse::<u16>().unwrap()) { current_password_counter += 1 }; },
                    "hgt" => { let mut ts = String::new(); let mut tn = String::new(); v.chars().for_each(|z| { if z.is_numeric() { tn.push(z) } else if z.is_alphabetic() { ts.push(z)} } ); match ts.as_str() { "in" => if (59..=76).contains(&tn.parse::<u16>().unwrap()) { current_password_counter += 1 }, "cm" => if (150..=193).contains(&tn.parse::<u16>().unwrap()) { current_password_counter += 1 }, _ => {}} },
                    "hcl" => { if v.len() == 7 && v.as_bytes()[0] == '#' as u8 { v.remove(0); if v.chars().all(char::is_alphanumeric) { current_password_counter += 1 } } },
                    "ecl" => { if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()) { current_password_counter += 1 } },
                    "pid" => { if v.len() == 9 && v.chars().all(char::is_numeric) { current_password_counter += 1 } },
                    "cid" => ( ),
                    _ => ()
                }
                // if prev == current_password_counter {
                //     println!("❌");
                // } else {
                //     println!("✅");
                // }
            });
            // println!("{}\n", current_password_counter);
            
            if current_password_counter >= 7 {
                valid_password_counter += 1;
            }
            current_password_counter = 0;
            temp_str.clear();
        }
    }
    println!("Answer: {}", valid_password_counter);
}
