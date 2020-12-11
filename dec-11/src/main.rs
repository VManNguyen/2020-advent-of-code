use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use grid::Grid;

fn is_occupied(spot : char) -> u8 {
    if spot == '#' {
        return 1;
    }
    0
}

fn seat_state(waiting_room : & Grid::<char>, 
              row : usize, 
              col : usize) -> char {
    let mut occupied_seats : u8 = 0;
    let sizes = waiting_room.size();

    let current_state : char = *(waiting_room.get(row, col).unwrap());
    if current_state == '.' {
        return '.';
    }

    if row == 0 {
        if col == 0 {
            occupied_seats += 
                is_occupied(*(waiting_room.get(row,  col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col  ).unwrap()));
        } else if col == sizes.1 - 1 {
            occupied_seats +=
                is_occupied(*(waiting_room.get(row,  col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col  ).unwrap()));
        } else {
            occupied_seats +=
                is_occupied(*(waiting_room.get(row,  col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col  ).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row,  col+1).unwrap()));
        }
    } else if row == sizes.0 - 1 {
        if col == 0 {
            occupied_seats += 
                is_occupied(*(waiting_room.get(row,  col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col  ).unwrap()));
        } else if col == sizes.1 - 1 {
            occupied_seats +=
                is_occupied(*(waiting_room.get(row,  col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col  ).unwrap()));
        } else {
            occupied_seats +=
                is_occupied(*(waiting_room.get(row,  col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col  ).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row,  col+1).unwrap()));
        }
    } else {
        if col == 0 {
            occupied_seats +=
                is_occupied(*(waiting_room.get(row-1,col  ).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row,  col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col  ).unwrap()));
        } else if col == sizes.1 - 1 {
            occupied_seats +=
                is_occupied(*(waiting_room.get(row,  col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col  ).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col  ).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col-1).unwrap()));
        } else {
            occupied_seats +=
                is_occupied(*(waiting_room.get(row,  col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col-1).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col  ).unwrap())) +
                is_occupied(*(waiting_room.get(row-1,col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row,  col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col+1).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col  ).unwrap())) +
                is_occupied(*(waiting_room.get(row+1,col-1).unwrap()));
        }
    }

    if current_state == '#' {   // If the seat is occupied
        if occupied_seats > 3 { // And if 4 adjacent seats are too
            return 'L';         // Then the dude is leaving his seat
        }
    } else {                    // Otherwise, if it is empty
        if occupied_seats == 0 {// And if nobody is around
            return '#';         // Then sit there
        }
    }

    current_state
}

fn simulate_waiting_room(waiting_room : &mut Grid::<char>) {
    let mut count : u64 = 0;
    let mut change : bool = true;
    let mut waiting_room_next = waiting_room.clone();
    while change {
        count += 1;
        change = false;
        let size = waiting_room.size();
        for i in 0..size.0 { // rows
            for j in 0..size.1 { // cols
                let next_state = seat_state(waiting_room, i, j);
                if next_state != waiting_room[i][j] {
                    waiting_room_next[i][j] = next_state;
                    change = true;
                }
            }
        }
        *waiting_room = waiting_room_next.clone();
    }

    println!("{} iterations", count);
    println!("Occupied seats: {}", count_occupied(waiting_room));
}

fn fov_n(waiting_room : & Grid::<char>, 
              row : usize, 
              col : usize) -> u8 {
    if row == 0 { // There's nothing north
        return 0;
    }

    let mut i = row - 1;
    while i >= 0 {
        let seat = waiting_room[i][col];
        if seat == '#' {
            return 1;
        } else if seat == 'L' {
            return 0;
        }
        if i == 0 { break; }
        i -= 1;
    }
    0
}

fn fov_s(waiting_room : & Grid::<char>, 
              row : usize, 
              col : usize) -> u8 {
    if row == waiting_room.size().0 - 1 { // There's nothing south
        return 0;
    }

    let mut i = row + 1;
    while i < waiting_room.size().0 {
        let seat = waiting_room[i][col];
        if seat == '#' {
            return 1;
        } else if seat == 'L' {
            return 0;
        }
        i += 1;
    }
    0
}

fn fov_w(waiting_room : & Grid::<char>,
         row : usize,
         col : usize) -> u8 {
    if col == 0 { // There's nothing west
        return 0;
    }

    let mut i = col - 1;
    while i >= 0 {
        let seat = waiting_room[row][i];
        if seat == '#' {
            return 1;
        } else if seat == 'L' {
            return 0;
        }
        if i == 0 { break; }
        i -= 1;
    }
    0
}

fn fov_e(waiting_room : & Grid::<char>,
         row : usize,
         col : usize) -> u8 {
    if col == waiting_room.size().1 - 1 { // There's nothing east
        return 0;
    }

    let mut i = col + 1;
    while i < waiting_room.size().1 {
        let seat = waiting_room[row][i];
        if seat == '#' {
            return 1;
        } else if seat == 'L' {
            return 0;
        }
        i += 1;
    }
    0
}

fn fov_nw(waiting_room : & Grid::<char>,
         row : usize,
         col : usize) -> u8 {
    if col == 0 || row == 0 { // There's nothing north-west
        return 0;
    }

    let mut i = row - 1;
    let mut j = col - 1;
    while i >= 0 && j >= 0 {
        let seat = waiting_room[i][j];
        if seat == '#' {
            return 1;
        } else if seat == 'L' {
            return 0;
        }
        if i == 0 || j == 0 { break; }
        i -= 1;
        j -= 1;
    }
    0
}

fn fov_sw(waiting_room : & Grid::<char>,
         row : usize,
         col : usize) -> u8 {
    if col == 0 || row == waiting_room.size().0 {
        return 0;
    }

    let mut i = row + 1;
    let mut j = col - 1;
    while i < waiting_room.size().0 && j >= 0 {
        let seat = waiting_room[i][j];
        if seat == '#' {
            return 1;
        } else if seat == 'L' {
            return 0;
        }
        if j == 0 { break; }
        i += 1;
        j -= 1;
    }
    0
}

fn fov_se(waiting_room : & Grid::<char>,
         row : usize,
         col : usize) -> u8 {
    if col == waiting_room.size().1 || row == waiting_room.size().0 {
        return 0;
    }

    let mut i = row + 1;
    let mut j = col + 1;
    while i < waiting_room.size().0 && j < waiting_room.size().1 {
        let seat = waiting_room[i][j];
        if seat == '#' {
            return 1;
        } else if seat == 'L' {
            return 0;
        }
        i += 1;
        j += 1;
    }
    0
}

fn fov_ne(waiting_room : & Grid::<char>,
         row : usize,
         col : usize) -> u8 {
    if col == waiting_room.size().1 || row == 0 {
        return 0;
    }

    let mut i = row - 1;
    let mut j = col + 1;
    while i >= 0 && j < waiting_room.size().1 {
        let seat = waiting_room[i][j];
        if seat == '#' {
            return 1;
        } else if seat == 'L' {
            return 0;
        }
        if i == 0 { break; }
        i -= 1;
        j += 1;
    }
    0
}

fn seat_state_fov(waiting_room : & Grid::<char>, 
              row : usize, 
              col : usize) -> char {
    let mut occupied_seats : u8 = 0;

    let current_state : char = *(waiting_room.get(row, col).unwrap());
    if current_state == '.' {
        return '.';
    }

    occupied_seats +=
        fov_n(&waiting_room, row, col) +
        fov_w(&waiting_room, row, col) +
        fov_s(&waiting_room, row, col) +
        fov_e(&waiting_room, row, col) +
        fov_nw(&waiting_room, row, col) +
        fov_sw(&waiting_room, row, col) +
        fov_se(&waiting_room, row, col) +
        fov_ne(&waiting_room, row, col);

    if current_state == '#' {   // If the seat is occupied
        if occupied_seats > 4 { // And if 4 adjacent seats are too
            return 'L';         // Then the dude is leaving his seat
        }
    } else {                    // Otherwise, if it is empty
        if occupied_seats == 0 {// And if nobody is around
            return '#';         // Then sit there
        }
    }

    current_state
}


fn simulate_waiting_room_fov(waiting_room : &mut Grid::<char>) {
    let mut count : u64 = 0;
    let mut change : bool = true;
    let mut waiting_room_next = waiting_room.clone();
    while change {
        count += 1;
        change = false;
        let size = waiting_room.size();
        for i in 0..size.0 { // rows
            for j in 0..size.1 { // cols
                let next_state = seat_state_fov(waiting_room, i, j);
                if next_state != waiting_room[i][j] {
                    waiting_room_next[i][j] = next_state;
                    change = true;
                }
            }
        }
        *waiting_room = waiting_room_next.clone();
    }

    println!("{} iterations", count);
    println!("Occupied seats: {}", count_occupied(waiting_room));
}

fn count_occupied(waiting_room : &Grid::<char>) -> u64 {
    let mut count : u64 = 0;
    for i in waiting_room.iter() {
        if *i == '#' {
            count += 1;
        }
    }
    count
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let lines : Vec<&str> = contents.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().collect::<Vec<char>>().len();
    let mut waiting_room = Grid::<char>::init(rows, cols, 'a');
    waiting_room.clear();

    for line in lines {
        waiting_room.push_row(line.chars().collect());
    }
    simulate_waiting_room(&mut waiting_room.clone());
    simulate_waiting_room_fov(&mut waiting_room.clone());
}
