use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"([a-z]*)\{(.+)\}").unwrap();
    let mut system = input.split("\n\r\n");

    let wrokflows: HashMap<String, Vec<(String, String, usize, String)>> = system
        .next()
        .unwrap()
        .split_whitespace()
        .map(|wrokflow| {
            let cap = re.captures(wrokflow).unwrap();
            let name = cap[1].to_string();
            let rules = cap[2]
                .to_string()
                .split(',')
                .map(|rule| {
                    if rule.contains(&['<', '>']) {
                        let parts = rule.split(':').collect::<Vec<_>>();
                        (
                            parts[0][0..1].to_string(),
                            parts[0][1..2].to_string(),
                            parts[0][2..].parse::<usize>().unwrap(),
                            parts[1].to_string(),
                        )
                    } else {
                        (rule.to_string(), rule.to_string(), 0, rule.to_string())
                    }
                })
                .collect::<Vec<_>>();
            (name, rules)
        })
        .collect();

    let ratings: Vec<HashMap<String, usize>> = system
        .next()
        .unwrap()
        .split_whitespace()
        .map(|rating| {
            let cap = re.captures(rating).unwrap();
            let rating = cap[2]
                .to_string()
                .split(',')
                .map(|rating| {
                    let item = rating.split('=').collect::<Vec<_>>();
                    (item[0].to_string(), item[1].parse::<usize>().unwrap())
                })
                .collect();
            rating
        })
        .collect();

    println!("Part 1: {}", solve_part_1(&wrokflows, &ratings));
    println!("Part 2: {}", solve_part_2(&wrokflows));
}

fn solve_part_1(
    wrokflows: &HashMap<String, Vec<(String, String, usize, String)>>,
    ratings: &Vec<HashMap<String, usize>>,
) -> usize {
    ratings
        .iter()
        .filter(|rating| is_accepted(wrokflows, rating, "in".to_string()))
        .map(|rating| rating.values().sum::<usize>())
        .sum()
}

fn is_accepted(
    wrokflows: &HashMap<String, Vec<(String, String, usize, String)>>,
    rating: &HashMap<String, usize>,
    key: String,
) -> bool {
    if key == "A" {
        return true;
    } else if key == "R" {
        return false;
    };
    for (src, op, val, dest) in wrokflows.get(&key).unwrap().iter() {
        if match_operation(rating, src, op, *val) {
            return is_accepted(wrokflows, rating, dest.to_string());
        }
    }
    unreachable!("No rule found for {}", key);
}

fn match_operation(rating: &HashMap<String, usize>, src: &String, op: &String, val: usize) -> bool {
    match op.as_str() {
        "<" => rating[src] < val,
        ">" => rating[src] > val,
        _ => true,
    }
}

fn solve_part_2(wrokflows: &HashMap<String, Vec<(String, String, usize, String)>>) -> usize {
    let ratings: HashMap<_, _> = "xmas"
        .chars()
        .map(|ch| (ch.to_string(), (1, 4000)))
        .collect();
    dfs(wrokflows, &ratings, "in".to_string())
}

fn dfs(
    wrokflows: &HashMap<String, Vec<(String, String, usize, String)>>,
    rating: &HashMap<String, (usize, usize)>,
    key: String,
) -> usize {
    if key == "A" {
        return rating
            .iter()
            .fold(1, |acc: usize, (_, (start, end))| acc * (end - start + 1));
    } else if key == "R" {
        return 0;
    }

    let mut ans = 0;
    let mut rating = rating.clone();

    for (src, op, val, dest) in wrokflows.get(&key).unwrap().iter() {
        if op != ">" && op != "<" {
            ans += dfs(wrokflows, &rating, dest.to_string());
            break;
        }

        let (start, end) = rating[src];
        if *val <= start || *val >= end {
            continue;
        };

        let [range, compliment] = split_range(op, *val, start, end);
        let mut next_ratings = rating.clone();

        rating.insert(src.to_string(), compliment);
        next_ratings.insert(src.to_string(), range);
        ans += dfs(wrokflows, &next_ratings, dest.to_string());
    }

    ans
}

fn split_range(op: &String, val: usize, start: usize, end: usize) -> [(usize, usize); 2] {
    match op.as_str() {
        ">" => [(val + 1, end), (start, val)],
        "<" => [(start, val - 1), (val, end)],
        _ => unreachable!("Invalid operation {}", op),
    }
}
