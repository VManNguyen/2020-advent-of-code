use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

fn translate(action : &str, val: i64, 
             x : &mut i64, y : &mut i64, hdg : u16) {
    let dir : &str;
    if action == "F" {
        dir = match hdg {
            0 => "N",
            90 => "E",
            180 => "S",
            270 => "W",
            _ => { println!("Error reading hdg: {}", hdg); return; }
        };
    } else {
        dir = action;
    }
    match dir {
        "N" => { *y += val; },
        "S" => { *y -= val; },
        "E" => { *x += val; },
        "W" => { *x -= val; },
        _ => {
            println!("Error in translation: {}{}", dir, val);
        }
    }
}

fn rotate(action : &str, val : i64, hdg : &mut u16) {
    let cur_hdg : i64 = *hdg as i64;

    match action {
        "R" => {
            *hdg = ((cur_hdg + val) % 360) as u16;
        },
        "L" => {
            if cur_hdg - val < 0 {
                *hdg = (360 + cur_hdg -val) as u16;
            } else {
                *hdg = (cur_hdg - val) as u16;
            }
        },
        _ => {
            println!("Error in rotation: {}{}", action, val);
        }
    };
}

fn simulate_boat(contents : &String) {
    let lines = contents.lines();
    let mut x : i64 = 0; // + east  | - west
    let mut y : i64 = 0; // + north | - south
    let mut hdg : u16 = 90; // 90 = east, 0 north, 180 south, 270 west
    
    let re = Regex::new(r"^(\w{1})(\d+)$").unwrap();

    for line in lines {
        let caps = match re.captures(line) {
            Some(inner) => inner,
            None => {
                println!("Failed to match: {}", line);
                return;
            }
        };

        let action : &str = caps.get(1).unwrap().as_str();
        let val : i64 = match caps.get(2).unwrap().as_str()
            .trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Could not read value: {}", line);
                    return;
                }
            };
        if action == "L" || action == "R" {
            rotate(action, val, &mut hdg);
        } else {
            translate(action, val, &mut x, &mut y, hdg);
        }
    }
    println!("Boat distance: {}", x.abs() + y.abs());
}

fn rotate_wpt(action : &str, val: i64, 
             x_w : &mut i64, y_w : &mut i64) {
    let angle : f64;
    match action {
        "R" => {
            angle = -val as f64;
        },
        "L" => {
            angle = val as f64;
        }
        _ => {
            println!("Error in rotation: {}{}", action, val);
            return;
        }
    }
    let angle = angle.to_radians();
    let sin = angle.sin() as i64;
    let cos = angle.cos() as i64;

    let x_rot = *x_w * cos - *y_w * sin;
    let y_rot = *x_w * sin + *y_w * cos;

    *x_w = x_rot;
    *y_w = y_rot;
}

fn translate_wpt(action : &str, val: i64, 
             x_w : &mut i64, y_w : &mut i64) {
    match action {
        "N" => { *y_w += val; },
        "S" => { *y_w -= val; },
        "E" => { *x_w += val; },
        "W" => { *x_w -= val; },
        _ => {
            println!("Error in translation: {}{}", action, val);
        }
    }
}

fn simulate_wpt(contents : &String) {
    let lines = contents.lines();
    let mut x_w : i64 = 10;
    let mut y_w : i64 = 1;
    let mut x : i64 = 0;
    let mut y : i64 = 0;
    
    let re = Regex::new(r"^(\w{1})(\d+)$").unwrap();

    for line in lines {
        let caps = match re.captures(line) {
            Some(inner) => inner,
            None => {
                println!("Failed to match: {}", line);
                return;
            }
        };

        let action : &str = caps.get(1).unwrap().as_str();
        let val : i64 = match caps.get(2).unwrap().as_str()
            .trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Could not read value: {}", line);
                    return;
                }
            };
        if action == "L" || action == "R" {
            rotate_wpt(action, val, &mut x_w, &mut y_w);
        } else if action == "F" {
            x = x + val * x_w;
            y = y + val * y_w;
        } else {
            translate_wpt(action, val, &mut x_w, &mut y_w);
        }
    }
    println!("Boat distance: {}", x.abs() + y.abs());
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");

    simulate_boat(&contents);
    simulate_wpt(&contents);
}
