use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();
    let mut gears: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut vals: HashMap<(usize, usize), i32> = HashMap::new();
    let (m, n) = (input.len(), input[0].len());

    let mut check_helper_helper = |i: usize, j: usize, k: (usize, usize)| {
        if input[i][j] == '*' {
            gears.entry((i, j)).or_insert(HashSet::new()).insert(k);
            true
        } else {
            false
        }
    };
    let mut check_helper = |i, j, k: (usize, usize)| {
        check_helper_helper(i, j, k)
            || (j > 0 && check_helper_helper(i, j - 1, k))
            || (j < n - 1 && check_helper_helper(i, j + 1, k))
    };
    let mut check = |i, j, k| {
        check_helper(i, j, k)
            || (i > 0 && check_helper(i - 1, j, k))
            || (i < m - 1 && check_helper(i + 1, j, k))
    };
    let mut ans = 0;
    let (mut condition, mut curr) = (false, 0);

    for (i, line) in input.iter().enumerate() {
        let mut k = 0;
        for (j, &ch) in line.iter().enumerate() {
            if ch.is_digit(10) {
                curr = curr * 10 + ch as i32 - '0' as i32;
                condition = condition || check(i, j, (i, k));
            } else {
                if condition {
                    vals.insert((i, k), curr);
                }
                condition = false;
                k = j + 1;
                curr = 0;
            }
        }

        if curr != 0 {
            if condition {
                vals.insert((i, k), curr);
            }
            condition = false;
            curr = 0;
        }
    }

    for v in gears.values() {
        if v.len() == 2 {
            ans += v.iter().fold(1, |acc, val| acc * vals[val]);
        }
    }

    println!("{}", ans);
}
