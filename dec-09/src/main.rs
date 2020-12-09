use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn part1(val : i64, window : &mut Vec<i64>) -> bool {
    let window_size = window.len();
    window.sort();

    for i in 0..window_size {
        for j in i+1..window_size {
            //println!("{} + {}", window[i], window[j]);
            if val == window[i] + window[j] {
                return true;
            } else if val < window[i] + window[j] {
                break;
            }
        }
    }

    false
}

fn part2(val : i64, v : &Vec<i64>) -> i64 {
    let mut i = 2;
    while i < v.len() - 1 {
        let iter = v.windows(i);
        for e in iter {
            if e.iter().sum::<i64>() == val {
                return e.iter().min().unwrap() + e.iter().max().unwrap();
            }
        }
        i += 1;
    }
    0
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let lines : Vec<i64> = contents.lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let window_size = 25;
    let mut incorrect_value = 0;

    for i in window_size..lines.len() {
        let val : i64 = lines[i];
        let mut window = lines[i-window_size..i].to_vec();

        if !part1(val, &mut window) {
            println!("{} not a sum of elements in window", val);
            incorrect_value = val;
            break;
        }
    }

    println!("min+max = {}", part2(incorrect_value, &lines));
}
