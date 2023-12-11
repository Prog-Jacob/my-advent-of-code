use std::collections::HashSet;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    const FACTOR: usize = 1000000;
    let mut rows: HashSet<usize> = HashSet::new();

    let universe = stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.unwrap();
            if line.chars().all(|ch| ch == '.') {
                rows.insert(i);
            }
            line
        })
        .collect::<Vec<_>>();

    let cols: HashSet<usize> = (0..universe[0].len())
        .filter(|&i| universe.iter().all(|row| &row[i..i + 1] == "."))
        .collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    universe.iter().enumerate().for_each(|(i, row)| {
        row.chars().enumerate().for_each(|(j, ch)| {
            if ch == '#' {
                galaxies.push((i, j));
            }
        });
    });

    let ans = galaxies.iter().fold(0, |acc, &(x1, y1)| {
        galaxies.iter().fold(acc, |acc, &(x2, y2)| {
            let (x1, x2) = (x1.min(x2), x1.max(x2));
            let (y1, y2) = (y1.min(y2), y1.max(y2));
            let (dx, dy) = (x2 - x1, y2 - y1);
            let empty_rows = (x1..x2).filter(|row| rows.contains(row)).count();
            let empty_cols = (y1..y2).filter(|col| cols.contains(col)).count();

            acc + dx + dy + (empty_rows * (FACTOR - 1)) + (empty_cols * (FACTOR - 1))
        })
    });

    println!("{}", ans / 2);
}
