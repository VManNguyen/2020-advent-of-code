use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

fn main() {
    let file = File::open("input").expect("Failed to read");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).expect("Failed to bufferize file");
    
    let re = Regex::new(r"^(\d+)-(\d+) (\w{1}): (\w+)$").unwrap();

    let mut res : i64 = 0;
    let lines = contents.lines();
    for line in lines {
        //println!("{}", line);
        for cap in re.captures_iter(line) {
            let inf : usize = match &cap[1].trim().parse() {
                Ok(num) => *num,
                Err(_) => break,
            };

            let sup : usize = match &cap[2].trim().parse() {
                Ok(num) => *num,
                Err(_) => break,
            };

            let c1 : char = match (&cap[4]).chars().clone().nth(inf - 1) {
                Some(inner) => inner,
                None => break,
            };
            let c2 : char = match (&cap[4]).chars().clone().nth(sup - 1) {
                Some(inner) => inner,
                None => break,
            };

            let mut bit : i64 = 0;
            if c1.to_string() == *(&cap[3]) {
                bit += 1;
            }
            if c2.to_string() == *(&cap[3]) {
                bit += 1;
            }

            println!("min: {} max: {} word:{} passwd:{} | c{}:{} c{}:{} bit:{}", &cap[1], &cap[2], &cap[3], &cap[4], inf-1, c1, sup-1, c2, bit);
            res+= bit % 2;
        }
    }
    println!("res: {}", res);
}
