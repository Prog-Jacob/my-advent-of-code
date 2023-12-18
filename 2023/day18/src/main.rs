use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let re = Regex::new(r"\(#([0-9a-f]{5})([0-3])\)").unwrap();
    let input: Vec<(i64, i64)> = stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let caps = re.captures_iter(&line).next().unwrap();
            let dist = i64::from_str_radix(&caps[1], 16).unwrap();
            let dir = caps[2].parse::<i64>().unwrap();
            (dist, dir)
        })
        .collect();

    println!("{}", solve(&input));
}

fn solve(input: &Vec<(i64, i64)>) -> i64 {
    let mut cols = BTreeMap::new();
    let mut rows = BTreeSet::new();
    let (mut i, mut j) = (0, 0);
    let mut area = 0;

    for &(dist, dir) in input {
        match dir {
            // Moving Right
            0 => {
                j += dist;
                area += dist;
                rows.insert(i);
            }
            // Moving Down
            1 => {
                i += dist;
                area += dist;
                cols.entry(j)
                    .or_insert(BTreeSet::new())
                    .insert((i - dist, i));
            }
            // Moving Left
            2 => {
                j -= dist;
                rows.insert(i);
            }
            // Moving Up
            3 => {
                i -= dist;
                cols.entry(j)
                    .or_insert(BTreeSet::new())
                    .insert((i, i + dist));
            }
            _ => unreachable!(),
        }
    }

    while !rows.is_empty() {
        let row = rows.pop_first().unwrap();

        loop {
            let mut height = i64::MAX;
            let mut intersections = Vec::new();

            for (&col, set) in &cols {
                if let Some(&(start, end)) = set.first() {
                    if start != row {
                        if intersections.len() == 1 {
                            height = height.min(start);
                        }
                        continue;
                    };
                    intersections.push((col, start, end));
                    height = height.min(end);
                }
                if intersections.len() == 2 {
                    break;
                }
            }

            if intersections.len() != 2 {
                break;
            };

            let width = intersections[1].0 - intersections[0].0;
            area += width * (height - row);

            for (col, _, end) in intersections.into_iter() {
                cols.get_mut(&col).unwrap().pop_first();
                if end != height {
                    cols.entry(col)
                        .or_insert(BTreeSet::new())
                        .insert((height, end));
                }
            }
        }
    }

    area + 1
}
