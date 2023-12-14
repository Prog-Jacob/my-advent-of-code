use std::collections::HashSet;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut pattern: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut set = HashSet::new();
    let mut stack = Vec::new();

    while set.insert(pattern.clone()) {
        stack.push(pattern.clone());
        roll_north(&mut pattern);
        roll_west(&mut pattern);
        roll_south(&mut pattern);
        roll_east(&mut pattern);
    }

    let cycle = stack.iter().position(|x| x == &pattern).unwrap();
    let ans = &stack[cycle + (1000000000 - cycle) % (stack.len() - cycle)];

    println!("{:?} and {cycle}", stack.len());
    println!("{}", get_load(ans));
}

fn get_load(pattern: &Vec<Vec<char>>) -> usize {
    let (m, n) = (pattern.len(), pattern[0].len());
    let mut ans = 0;

    for i in 0..m {
        for j in 0..n {
            if pattern[i][j] == 'O' {
                ans += m - i;
            }
        }
    }

    ans
}

fn roll_north(pattern: &mut Vec<Vec<char>>) {
    let (m, n) = (pattern.len(), pattern[0].len());

    for j in 0..n {
        let mut idx = 0;
        let mut dot_cnt = 0;

        for i in 0..m {
            if pattern[i][j] == 'O' {
                pattern[idx][j] = 'O';
                idx += 1;
            } else if pattern[i][j] == '#' {
                for k in 0..dot_cnt {
                    pattern[idx + k][j] = '.';
                }
                idx = i + 1;
                dot_cnt = 0;
            } else {
                dot_cnt += 1;
            }
        }

        for k in 0..dot_cnt {
            pattern[idx + k][j] = '.';
        }
    }
}

fn roll_west(pattern: &mut Vec<Vec<char>>) {
    let (m, n) = (pattern.len(), pattern[0].len());

    for i in 0..m {
        let mut idx = 0;
        let mut dot_cnt = 0;

        for j in 0..n {
            if pattern[i][j] == 'O' {
                pattern[i][idx] = 'O';
                idx += 1;
            } else if pattern[i][j] == '#' {
                for k in 0..dot_cnt {
                    pattern[i][idx + k] = '.';
                }
                idx = j + 1;
                dot_cnt = 0;
            } else {
                dot_cnt += 1;
            }
        }

        for k in 0..dot_cnt {
            pattern[i][idx + k] = '.';
        }
    }
}

fn roll_south(pattern: &mut Vec<Vec<char>>) {
    let (m, n) = (pattern.len(), pattern[0].len());

    for j in 0..n {
        let mut idx = m - 1;
        let mut dot_cnt = 0;

        for i in (0..m).rev() {
            if pattern[i][j] == 'O' {
                pattern[idx][j] = 'O';
                idx -= 1;
            } else if pattern[i][j] == '#' {
                for k in 0..dot_cnt {
                    pattern[idx - k][j] = '.';
                }
                idx = i - 1;
                dot_cnt = 0;
            } else {
                dot_cnt += 1;
            }
        }

        for k in 0..dot_cnt {
            pattern[idx - k][j] = '.';
        }
    }
}

fn roll_east(pattern: &mut Vec<Vec<char>>) {
    let (m, n) = (pattern.len(), pattern[0].len());

    for i in 0..m {
        let mut idx = n - 1;
        let mut dot_cnt = 0;

        for j in (0..n).rev() {
            if pattern[i][j] == 'O' {
                pattern[i][idx] = 'O';
                idx -= 1;
            } else if pattern[i][j] == '#' {
                for k in 0..dot_cnt {
                    pattern[i][idx - k] = '.';
                }
                idx = j - 1;
                dot_cnt = 0;
            } else {
                dot_cnt += 1;
            }
        }

        for k in 0..dot_cnt {
            pattern[i][idx - k] = '.';
        }
    }
}
