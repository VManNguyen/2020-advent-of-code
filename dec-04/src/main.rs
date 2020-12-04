use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

fn is_valid_year(year: &str, inf: i64, sup: i64) -> bool {
    let check : i64 = match year.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return false;
        }
    };
    
    if check <= sup && check >= inf {
        return true;
    }

    false
}

fn is_valid_hgt(hgt: &str) -> bool {
    let re = Regex::new(r"(\d+)(in|cm)").unwrap();
    let caps = match re.captures(hgt) {
        Some(inner) => inner,
        None => {
            return false;
        }
    };

    let val : i64 = match caps.get(1).unwrap().as_str().trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return false;
        }
    };

    let unit : &str = caps.get(2).unwrap().as_str();
    let unit = String::from(unit);
    if unit == "in" {
        if val <= 76 && val >= 59 {
            return true;
        }
    } else if unit == "cm" {
        if val <= 193 && val >= 150 {
            return true;
        }
    }

    false
}

fn is_valid_hcl(hcl: &str) -> bool {
    let re = Regex::new(r"#([0-9a-f]{6})").unwrap();
    match re.captures(hcl) {
        Some(_) => { return true; },
        None => { return false; }
    };
}

fn is_valid_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => { return true; },
        _ => { return false; }
    };
}

fn is_valid_pid(pid: &str) -> bool {
    let re = Regex::new(r"(\d{9})").unwrap();
    match re.captures(pid) {
        Some(_) => {
            if pid.len() == 9 {
                return true;
            } else {
                return false;
            }
        },
        _ => { return false; }
    };
}

fn is_valid_passport(passport: String) -> bool {
    let mut byr: bool = false; // birth year
    let mut iyr: bool = false; // issue year
    let mut eyr: bool = false; // expi. year
    let mut hgt: bool = false; // height
    let mut hcl: bool = false; // hair color
    let mut ecl: bool = false; // eye color
    let mut pid: bool = false; // passport id

    //println!("Passport: ");
    //println!("{}", passport);

    let re = Regex::new(r"^(\w{3}):(#?[A-Za-z0-9]*)").unwrap();
    for line in passport.lines() {
        let caps = match re.captures(line) {
            Some(inner) => inner,
            None => continue,
        };
        match caps.get(1).unwrap().as_str() {
            "byr" => byr = is_valid_year(
                caps.get(2).unwrap().as_str(),
                1920,
                2002
            ),
            "iyr" => iyr= is_valid_year(
                caps.get(2).unwrap().as_str(),
                2010,
                2020
            ),
            "eyr" => eyr = is_valid_year(
                caps.get(2).unwrap().as_str(),
                2020,
                2030
            ),
            "hgt" => hgt = is_valid_hgt(
                caps.get(2).unwrap().as_str()
            ),
            "hcl" => hcl = is_valid_hcl(
                caps.get(2).unwrap().as_str()
            ),
            "ecl" => ecl = is_valid_ecl(
                caps.get(2).unwrap().as_str()
            ),
            "pid" => pid = is_valid_pid(
                caps.get(2).unwrap().as_str()
            ),
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
