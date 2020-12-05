use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn seat_id(code: &str) -> u64 {
    let mut row_min : u64 = 0;
    let mut row_max : u64 = 127;
    let mut col_min : u64 = 0;
    let mut col_max : u64 = 7;

    //println!("row_mid: {}, col_mid: {}", row_mid, col_mid);
    for c in code.chars() {
        let row_mid = (row_max - row_min + 1) / 2;
        let col_mid = (col_max - col_min + 1) / 2;
        match c {
            'F' => {
                row_max -= row_mid;
            },
            'B' => {
                row_min += row_mid;
            },
            'R' => {
                col_min += col_mid;
            },
            'L' => {
                col_max -= col_mid;
            }
            _ => { continue; }
        };
    }

    row_min * 8 + col_min
}

fn find_seat(mut v : Vec<u64>) -> u64 {
    v.sort();

    for i in 1..v.len() {
        if v[i] - v[i-1] != 1 {
            return v[i] - 1;
        }
    }

    0
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let lines = contents.lines();

    let mut max_id : u64 = 0;
    let mut v : Vec<u64> = Vec::new();
    for line in lines {
        let id = seat_id(line);
        v.push(id);
        if max_id < id {
            max_id = id;
        }
    }
    
    println!("Max seat id: {}", max_id);
    println!("Seat id: {}", find_seat(v));
}
