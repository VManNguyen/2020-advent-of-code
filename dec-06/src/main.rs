use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn count_answers(group : String) -> i64 {
    println!("New group: {}", group);

    let mut char_vec : Vec<char> = group.chars().collect();
    char_vec.sort();
    char_vec.dedup();
    
    char_vec.len() as i64
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let lines = contents.lines();
    let mut count : i64 = 0;

    let mut group = String::new();
    for line in lines {
        group.push_str(line.trim());
        if line.is_empty() {
            count += count_answers(group);
            group = String::new();
        }
    }
    count += count_answers(group);

    println!("Count: {}", count);
}
