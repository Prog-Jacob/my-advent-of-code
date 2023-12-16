use std::collections::{HashSet, VecDeque};
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let tiles: Vec<Vec<char>> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (m, n) = (tiles.len(), tiles[0].len());
    let mut ans = 0;

    (0..m).for_each(|i| {
        ans = ans.max(get_heat_value(&tiles, i, n - 1, 0, usize::MAX));
        ans = ans.max(get_heat_value(&tiles, i, 0, 0, 1));
    });
    (0..n).for_each(|j| {
        ans = ans.max(get_heat_value(&tiles, m - 1, j, usize::MAX, 0));
        ans = ans.max(get_heat_value(&tiles, 0, j, 1, 0));
    });

    println!("{ans}");
}

fn get_heat_value(tiles: &Vec<Vec<char>>, i: usize, j: usize, x: usize, y: usize) -> usize {
    let mut queue: VecDeque<(usize, usize, usize, usize)> = VecDeque::from([(i, j, x, y)]);
    let mut visited: HashSet<(usize, usize, usize, usize)> = HashSet::new();

    while !queue.is_empty() {
        let (i, j, x, y) = queue.pop_front().unwrap();
        if i >= tiles.len() || j >= tiles[0].len() || !visited.insert((i, j, x, y)) {
            continue;
        }

        match tiles[i][j] {
            '|' if y == 1 || y == usize::MAX => {
                queue.push_back((i - 1, j, usize::MAX, 0));
                queue.push_back((i + 1, j, 1, 0));
            }
            '-' if x == 1 || x == usize::MAX => {
                queue.push_back((i, j - 1, 0, usize::MAX));
                queue.push_back((i, j + 1, 0, 1));
            }
            '.' | '|' | '-' => queue.push_back((i + x, j + y, x, y)),
            '/' => queue.push_back((i - y, j - x, 0 - y, 0 - x)),
            _ => queue.push_back((i + y, j + x, y, x)),
        }
    }

    visited
        .iter()
        .map(|(i, j, _, _)| (i, j))
        .collect::<HashSet<_>>()
        .len()
}
