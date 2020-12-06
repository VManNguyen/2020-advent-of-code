use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let lines = contents.lines();
    let mut count : usize = 0;
    let mut reset : bool = true;

    let mut group = HashSet::new();
    for line in lines {
        if line.is_empty() {
            count += group.len();
            reset = true;
        } else {
            let h = HashSet::from_iter(line.chars());
            if reset {
                group = h.clone();
                reset = false;
            } else {
                group = HashSet::from_iter(group
                                           .intersection(&h).cloned());
            }
        }
    }
    count += group.len();

    println!("Count: {}", count);
}
