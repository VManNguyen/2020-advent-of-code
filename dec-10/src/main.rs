use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::collections::HashMap;

use petgraph::graphmap::DiGraphMap;
use petgraph::Incoming;
//use petgraph::dot::{Dot, Config};

fn part_1(adapters : Vec<i64>, outlet : i64) -> i64 {
    let mut n_1 : i64 = 0;
    let mut n_3 : i64 = 1; // Final adapter -> device is always 3
    let diff = adapters[0] - outlet;
    match diff {
        1 => n_1 += 1,
        2 => println!("Diff 2"),
        3 => n_3 += 1,
        _ => println!("Error"),
    };

    for i in 1..adapters.len() {
        let diff = adapters[i] - adapters[i-1];
        match diff {
            1 => n_1 += 1,
            2 => println!("Diff 2"),
            3 => n_3 += 1,
            _ => println!("Error"),
        };
    }

    n_1 * n_3
}

fn dfs(g : &DiGraphMap<i64, i64>, 
       val : i64, 
       max : i64, 
       mut c : &mut i64) {
    if val == max {
        *c += 1;
        return
    }

    for n in g.neighbors(val) {
        dfs(g, n, max, &mut c);
    }
}

fn intersect(b1 : i64,
             b2 : i64,
             idoms : &HashMap<i64, i64>) -> i64 {
    let mut finger1 = &b1;
    let mut finger2 = &b2;

    while finger1 != finger2 {
        while finger1 > finger2 {
            finger1 = idoms.get(finger1).unwrap();
        }
        while finger2 > finger1 {
            finger2 = idoms.get(finger2).unwrap();
        }
    }
    *finger1
}

fn build_idoms(g : &DiGraphMap<i64, i64>,
            root : i64) -> HashMap<i64, i64> {
    let mut idoms = HashMap::<i64, i64>::new();

    idoms.insert(root, root);
    let mut changed : bool = true;
    while changed {
        changed = false;
        let mut nodes = g.nodes().collect::<Vec<i64>>();
        nodes.sort();
        for b in nodes {
            if b == root { continue; }
            let preds : Vec<i64> = g
                .neighbors_directed(b, Incoming)
                .collect();
            let mut new_idom = preds[0]; // just pick one
            for p in 1..preds.len() {
                if idoms.contains_key(&(preds[p])) {
                    new_idom = intersect(preds[p], new_idom, &idoms);
                }
            }
            if !(idoms.contains_key(&b)) 
                || *(idoms.get(&b).unwrap()) != new_idom {
                    idoms.insert(b, new_idom);
                    changed = true;
            }
        }
    }

    idoms
}

fn part_2(adapters : Vec<i64>, outlet : i64) -> i64 {
    let mut g = DiGraphMap::<i64, i64>::new();
    for adapter in &adapters[0..3] {
        let diff = adapter - outlet;
        if diff <= 3 {
            g.add_edge(outlet, *adapter, diff);
        }
    }
    let device = adapters.iter().max().unwrap() + 3;
    for adapter in &adapters[adapters.len()-3..] {
        let diff = device - adapter;
        if diff <= 3 {
            g.add_edge(*adapter, device, diff);
        } 
    }

    for i in 0..3 {
        for j in i+1..3 {
            let diff = adapters[j] - adapters[i];
            if diff <= 3 {
                g.add_edge(adapters[i], adapters[j], diff);
            }
        }
    }

    for i in 3..adapters.len() {
        for prev in &adapters[i-3..i] {
            let diff = adapters[i] - prev;
            if diff <= 3 {
                g.add_edge(*prev, adapters[i], diff);
            } 
        }
    }
    //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let mut count = 1;
    let idoms : HashMap<i64, i64> = build_idoms(&g, outlet);

    let mut iter : i64 = device;
    while iter != outlet {
        let mut low_count : i64 = 0;
        let low = idoms.get(&iter).unwrap();
        dfs(&g, *low, iter, &mut low_count);
        count *= low_count;
        iter = *low;
    }

    
    count
}

fn main() {
    let file = File::open("input").expect("Failed to read file input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect("Failed to bufferize file input");
    let mut lines : Vec<i64> = contents.lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    lines.sort();
    
    let outlet : i64 = 0;

    println!("Part 1: {}", part_1(lines.clone(), outlet));
    println!("Part 2: {}", part_2(lines, outlet));
}
