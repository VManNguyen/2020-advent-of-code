use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::collections::HashSet;

use regex::Regex;

use petgraph::graphmap::DiGraphMap;
use petgraph::Incoming;
//use petgraph::dot::{Dot, Config};

fn part1(g : &DiGraphMap<&str, i64>) -> i64 {
    let mut count : i64 = 0;
    let mut to_visit : Vec<&str> = g.neighbors("shiny gold").collect();
    let mut visited : HashSet<&str> =  HashSet::new();
    while !to_visit.is_empty() {
        let elem : &str = match to_visit.pop() {
            Some(inner) => inner,
            None => break,
        };
        if visited.contains(elem) {
            // Really wanted to be sure
            continue;
        }
        visited.insert(elem);
        count += 1;
        for n in g.neighbors(elem) {
            if !visited.contains(n) {
                to_visit.push(n);
            }
        }
    }
    
    count
}

fn part2(g : &DiGraphMap<&str, i64>,
         colour : &str) -> i64 {
    let mut content : i64 = 0;
    let to_visit : Vec<&str> = g.neighbors_directed(colour, Incoming)
        .collect();
    if to_visit.is_empty() {
        return content;
    }

    for node in to_visit {
        let weight = match g.edge_weight(node, colour) {
            Some(inner) => inner,
            None => continue,
        };
        content += weight + weight * part2(g, node);
    }

    content
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let lines = contents.lines();

    let re_container = Regex::new(r"^(.*?) bags contain (\d .*).")
        .unwrap();
    let re_content = Regex::new(r"^(\d+) (.*?) bag")
        .unwrap();

    let mut g = DiGraphMap::<&str, i64>::new();

    for line in lines {
        let caps = match re_container.captures(line) {
            Some(inner) => inner,
            None => continue,
        };
        let container = caps.get(1).unwrap().as_str();
        let content : Vec<&str> = caps
            .get(2).unwrap().as_str().split(',').collect();
        for elem in content {
            let caps_content = match re_content.captures(elem.trim()) {
                Some(inner) => inner,
                None => continue,
            };
            let num : i64 = match caps_content.get(1)
                .unwrap().as_str().trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
            let colour = caps_content.get(2).unwrap().as_str();
            g.add_edge(colour, container, num);
        }
        //add_edge(&mut g, container, content);
    }


    println!("Count: {}", part1(&g));
    println!("Count: {}", part2(&g, "shiny gold"));

    //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}
