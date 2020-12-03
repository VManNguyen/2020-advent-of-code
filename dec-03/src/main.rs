use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn slope(lines: &mut std::str::Lines,
          lines_len: usize,
          right: usize,
          down: usize) -> i64 {
    let mut trees = 0;
    let mut line_num : usize = 0;

    for line in lines {
        line_num += 1;
        if line_num % down != 0 {
            continue;
        }

        let access : usize = (line_num/down * right) % lines_len;
        //println!("check {}[{}]: {}", line_num, access, line);

        let c : char = match line
            .chars()
            .clone()
            .nth(access) {
                Some(inner) => inner,
                None => break,
            };
        if c == '#' {
            trees += 1;
        }
    }

    trees
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let mut lines = contents.lines();

    let lines_len : usize = lines.next().unwrap().len();
    println!("lines length: {}", lines_len);

    //println!("R1D1: {}", slope(&mut lines.clone(), lines_len, 1, 1));
    //println!("R3D1: {}", slope(&mut lines.clone(), lines_len, 3, 1));
    //println!("R5D1: {}", slope(&mut lines.clone(), lines_len, 5, 1));
    //println!("R7D1: {}", slope(&mut lines.clone(), lines_len, 7, 1));
    //println!("R1D2: {}", slope(&mut lines.clone(), lines_len, 1, 2));

    let trees : i64 = slope(&mut lines.clone(), lines_len, 1, 1)
        * slope(&mut lines.clone(), lines_len, 3, 1)
        * slope(&mut lines.clone(), lines_len, 5, 1)
        * slope(&mut lines.clone(), lines_len, 7, 1)
        * slope(&mut lines.clone(), lines_len, 1, 2);
    println!("trees: {}", trees);
}
