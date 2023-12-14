use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut ans = 0;
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let patterns = input
        .split("\n\r\n")
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for pattern in patterns.iter() {
        let transpose: Vec<String> = (0..pattern[0].len())
            .map(|i| pattern.iter().map(|s| &s[i..=i]).collect())
            .collect();
        ans += reflections(&transpose);
        ans += reflections(&pattern) * 100;
    }

    println!("{ans}");
}

fn reflections(pattern: &Vec<String>) -> usize {
    let n = pattern.len();

    for i in 1..n {
        let j = i.min(n - i);
        let diff = (0..j).fold(0, |acc_o, k| {
            acc_o
                + pattern[i - k - 1]
                    .chars()
                    .zip(pattern[i + k].chars())
                    .fold(0, |acc_i, (a, b)| if a != b { acc_i + 1 } else { acc_i })
        });

        if diff == 1 {
            return i;
        }
    }

    0
}
