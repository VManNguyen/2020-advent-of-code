use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

fn is_valid_passport(passport: String) -> bool {
    let mut byr: bool = false; // birth year
    let mut iyr: bool = false; // issue year
    let mut eyr: bool = false; // expi. year
    let mut hgt: bool = false; // height
    let mut hcl: bool = false; // hair color
    let mut ecl: bool = false; // eye color
    let mut pid: bool = false; // passport id
    let mut cid: bool = false; // country id (opt.)

    //println!("Passport: ");
    //println!("{}", passport);

    let re = Regex::new(r"^(\w{3}):(#?[A-Za-z0-9]*)").unwrap();
    for line in passport.lines() {
        let caps = match re.captures(line) {
            Some(inner) => inner,
            None => continue,
        };
        match caps.get(1).unwrap().as_str() {
            "byr" => byr=true,
            "iyr" => iyr=true,
            "eyr" => eyr=true,
            "hgt" => hgt=true,
            "hcl" => hcl=true,
            "ecl" => ecl=true,
            "pid" => pid=true,
            "cid" => cid=true,
            _ => continue,
        };
        //println!("cap1: {}", caps.get(1).unwrap().as_str());
    }

    return byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let lines = contents.lines();

    let mut count : i64 = 0;
    let mut passport = String::new();
    for line in lines {
        passport.push_str(line.trim());
        passport.push_str(" ");
        if line.is_empty() {
            if is_valid_passport(passport.replace(" ", "\n")) {
                count += 1;
            }
            passport = String::new();
        }
    }
    // Checking the last one
    if is_valid_passport(passport.replace(" ", "\n")) {
        count += 1;
    }

    println!("Valid passports: {}", count);
}
