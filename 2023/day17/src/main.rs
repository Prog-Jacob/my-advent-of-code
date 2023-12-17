use std::collections::{BTreeSet, HashSet};
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let input: Vec<Vec<_>> = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    const DIRS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    // For Part 1: MAX_STEPS = 3 and MIN_STEPS = 1
    const MAX_STEPS: usize = 10;
    const MIN_STEPS: usize = 4;

    let mut queue = BTreeSet::from([(0, 0, 0, 0, 0), (0, 0, 0, 0, 1)]);
    let mut visited = HashSet::new();
    let (m, n) = (input.len(), input[0].len());

    while !queue.is_empty() {
        let (weight, i, j, curr_steps, curr_dir) = queue.pop_first().unwrap();
        if i == m - 1 && j == n - 1 && curr_steps >= MIN_STEPS {
            println!("{weight}");
            return;
        }
        visited.insert((i, j, curr_steps, curr_dir));

        for next_dir in 0..4 {
            if curr_steps < MIN_STEPS && next_dir != curr_dir {
                continue;
            };
            if next_dir == (curr_dir + 2) % 4 || (curr_steps >= MAX_STEPS && next_dir == curr_dir) {
                continue;
            }

            let next_steps = if next_dir == curr_dir {
                curr_steps + 1
            } else {
                1
            };
            let (di, dj) = DIRS[next_dir];
            let (x, y) = (i + di, j + dj);

            if x >= m || y >= n || visited.contains(&(x, y, next_steps, next_dir)) {
                continue;
            }
            queue.insert((weight + input[x][y], x, y, next_steps, next_dir));
        }
    }
}
