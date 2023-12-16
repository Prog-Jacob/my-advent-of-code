use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut adj = HashMap::new();
    let mut instructions: Vec<char> = Vec::new();

    for (i, line) in stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            instructions = line.chars().collect();
        } else if let Some(caps) = re.captures(&line) {
            adj.insert(
                caps[1].to_string(),
                (caps[2].to_string(), caps[3].to_string()),
            );
        }
    }

    let mut ans = 1;
    let n = instructions.len();

    for from in adj.keys().filter(|k| k.ends_with('A')) {
        let mut j = 0;
        let mut from = from;

        while !from.ends_with('Z') {
            if instructions[j % n] == 'L' {
                from = &adj[from].0;
            } else {
                from = &adj[from].1;
            }
            j += 1;
        }

        ans = lcm(ans, j);
    }

    println!("{ans}");
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a.max(b), a.min(b))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
