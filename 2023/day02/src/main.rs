use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut ans = 0;
    let mut cubes: HashMap<String, i32> = HashMap::new();
    let re1 = Regex::new(r"Game (\d+): (.*)").unwrap();
    let re2 = Regex::new(r"(\d+) (blue|green|red)").unwrap();

    for line in stdin().lock().lines() {
        if let Some(games) = re1.captures(&line.unwrap()) {
            games[2].split(&[';', ',']).for_each(|game| {
                if let Some(cube) = re2.captures(game) {
                    let num = cube[1].parse::<i32>().unwrap();
                    let c = cubes.entry(cube[2].to_string()).or_insert(0);
                    *c = max(*c, num);
                };
            });
        };

        if cubes.len() == 3 {
            ans += cubes.values().fold(1, |acc, v| acc * v);
        }
        cubes.clear();
    }

    println!("{ans}");
}
