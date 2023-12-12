use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut ans = 0;

    for line in stdin().lock().lines() {
        let line: String = line.unwrap();
        let recordings: Vec<&str> = line.split_whitespace().collect::<Vec<_>>();

        // Repeats the string five times, and join them with '?', then converts it into a char array.
        let gears: Vec<char> = recordings
            .clone()
            .into_iter()
            .take(1)
            .cycle()
            .take(5)
            .collect::<Vec<_>>()
            .join("?")
            .chars()
            .collect();

        // Repeats the string five times, and join them with ',', then parse the numbers into an array.
        let groups: Vec<usize> = recordings
            .into_iter()
            .skip(1)
            .cycle()
            .take(5)
            .collect::<Vec<_>>()
            .join(",")
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        ans += combinate(0, 0, &gears, &groups);
    }

    println!("{}", ans);
}

fn combinate(i: usize, j: usize, gears: &Vec<char>, groups: &Vec<usize>) -> i64 {
    let mut memo = vec![vec![-1; groups.len()]; gears.len()];
    dp(i, j, gears, groups, &mut memo)
}

fn dp(i: usize, j: usize, gears: &Vec<char>, groups: &Vec<usize>, memo: &mut Vec<Vec<i64>>) -> i64 {
    if j == groups.len() && gears.iter().skip(i).all(|&x| x != '#') {
        // All the groups are consumed and the rest of the gears are not damages (a.k.a. '#').
        return 1;
    } else if j >= groups.len() || i + groups[j] > gears.len() {
        // All the groups are consumed or the rest of the gears are not enough to consume.
        return 0;
    } else if memo[i][j] != -1 {
        // The result is already calculated.
        return memo[i][j];
    }

    memo[i][j] = 0;
    let end = i + groups[j];

    // Keep trying different starting points as long as you don't leave a damaged gear behind.
    if gears[i] != '#' {
        memo[i][j] += dp(i + 1, j, gears, groups, memo);
    }
    // Consider this candidate when:
    //// 1. The required window of gears are all damaged ('#' for damaged, '?' for wildcard).
    //// 2. Either all the gears are consumed or the next gear is not damaged.
    if gears.iter().take(end).skip(i).all(|&x| x != '.')
        && (end == gears.len() || gears[end] != '#')
    {
        memo[i][j] += dp(end + 1, j + 1, gears, groups, memo);
    }

    memo[i][j]
}
