use std::collections::HashSet;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let lines = stdin().lock().lines().enumerate().collect::<Vec<_>>();
    let mut boards = vec![1; lines.len()];

    for (i, line) in lines {
        let line = line.unwrap();
        let mut cards = line.split(&[':', '|']).skip(1);
        let wins = cards
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let total = cards
            .next()
            .unwrap()
            .split_whitespace()
            .filter(|x| wins.contains(&x.parse::<u32>().unwrap()))
            .count();

        for idx in 1..=total {
            boards[i + idx] += boards[i];
        }
    }

    println!("{}", boards.iter().sum::<u32>());
}
