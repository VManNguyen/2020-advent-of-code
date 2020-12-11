use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::collections::HashSet;

use regex::Regex;

fn acc_instr(op : &str, val : i64) -> i64 {
    if op == "+" {
        return val;
    } else {
        return -val;
    }
}

fn jmp_instr(op : &str, val : i64, curr : usize) -> usize {
    let n : i64 = acc_instr(op, val);
    if n.is_negative() {
        return match curr.checked_sub(n.wrapping_abs() as usize) {
                Some(inner) => inner,
                None => 0,
        };
    } else {
        return match curr.checked_add(val as usize) {
            Some(inner) => inner,
            None => 0,
        };
    }
}

fn get_operands(line : &str) -> (&str, &str, i64) {
    let re = Regex::new(r"^(jmp|acc|nop){1} (\+|-){1}(\d+)$").unwrap();

    let caps = match re.captures(line) {
        Some(inner) => inner,
        None => {
            println!("Failed to match: {}", line);
            return ("false", "false", -1);
        }
    };
    let instruction : &str = caps.get(1).unwrap().as_str();
    let op : &str = caps.get(2).unwrap().as_str();
    let val : i64 = match caps.get(3)
        .unwrap().as_str().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to read number: {}", line);
                return ("false", "false", -1);
            }
        };

    (instruction, op, val)
}

fn get_acc(lines : Vec<&str>) -> i64 {
    let mut i : usize = 0;
    let mut acc : i64 = 0;
    let mut visited : HashSet<usize>= HashSet::new();


    while i < lines.len() {
        visited.insert(i);
        //println!("Executing: {}", lines[i]);

        let operands = get_operands(lines[i]);
        if operands == ("false", "false", -1) {
            break;
        }

        match operands.0 {
            "nop" => {i += 1},
            "acc" => {
                acc += acc_instr(operands.1, operands.2);
                i += 1;
            },
            "jmp" => {
                i = jmp_instr(operands.1, operands.2, i);
            }
            _ => {
                println!("Error reading line: {}", lines[i]);
                break;
            }
        };

        if visited.contains(&i) {
            println!("Stopped at instruction {}: {}", i, lines[i]);
            break;
        }
    }

    acc
}

fn get_fixed_acc(lines : Vec<&str>) -> i64 {
    let mut i : usize = 0;
    let mut acc : i64 = 0;
    let mut visited : HashSet<usize> = HashSet::new();


    while i < lines.len() {
        visited.insert(i);
        //println!("Executing: {}", lines[i]);

        let operands = get_operands(lines[i]);
        if operands == ("false", "false", -1) {
            break;
        }

        match operands.0 {
            "nop" => {
                let path_results : (bool, i64) 
                    = explore_path(lines.clone(), i, acc, visited.clone());
                if path_results.0 {
                    println!("Flipped instruction {}: {}", i, lines[i]);
                    return path_results.1;
                }
                i += 1;
            },
            "acc" => {
                acc += acc_instr(operands.1, operands.2);
                i += 1;
            },
            "jmp" => {
                let path_results : (bool, i64) 
                    = explore_path(lines.clone(), i, acc, visited.clone());
                if path_results.0 {
                    println!("Flipped instruction {}: {}", i, lines[i]);
                    return path_results.1;
                }
                i = jmp_instr(operands.1, operands.2, i);
            }
            _ => {
                println!("Error reading line: {}", lines[i]);
                break;
            }
        };

        if visited.contains(&i) {
            println!("Stopped at instruction {}: {}", i, lines[i]);
            break;
        }
    }

    acc
}

fn explore_path(lines : Vec<&str>, 
                mut i : usize, 
                mut acc : i64,
                visited : HashSet<usize>) -> (bool, i64) {
    let mut visited_ : HashSet<usize> = visited.clone();
    visited_.insert(i);

    let operands = get_operands(lines[i]);
    if operands == ("false", "false", -1) {
        return (false, 0);
    }
    match operands.0 {
        "nop" => {
            i = jmp_instr(operands.1, operands.2, i);
        },
        "jmp" => {
            i += 1;
        },
        _ => {
            println!("Error in exploring path");
            return (false, 0);
        }
    };

    while i < lines.len() {
        visited_.insert(i);
        //println!("Executing: {}", lines[i]);

        let operands = get_operands(lines[i]);
        if operands == ("false", "false", -1) {
            break;
        }

        match operands.0 {
            "nop" => {i += 1},
            "acc" => {
                acc += acc_instr(operands.1, operands.2);
                i += 1;
            },
            "jmp" => {
                i = jmp_instr(operands.1, operands.2, i);
            }
            _ => {
                println!("Error reading line: {}", lines[i]);
                break;
            }
        };

        if visited_.contains(&i) {
            //println!("Stopped at instruction {}: {}", i, lines[i]);
            return (false, 0);
        }
    }
    
    (true, acc)
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let lines : Vec<&str> = contents.lines().collect();

    println!("acc: {}", get_acc(lines.clone()));
    println!("fix acc: {}", get_fixed_acc(lines.clone()));
}
