use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let file = File::open("input").expect("Failed to read");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).expect("Failed to bufferize file");

    let lines = contents.lines();

    for i in 0..lines.clone().count() {
        let a : i64 = match lines.clone().nth(i).unwrap_or("-1").trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        for j in i..lines.clone().count() {
            let b : i64 = match lines.clone().nth(j).unwrap_or("-1").trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            for k in j..lines.clone().count() {
                let c : i64 = match lines.clone().nth(k).unwrap_or("-1").trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if a + b + c == 2020 {
                    println!("{}", a*b*c);
                    break;
                }
            }
        }
    }
}
