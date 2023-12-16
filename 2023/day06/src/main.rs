use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let input: Vec<usize> = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .replace(" ", "")
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    let (k, v) = (input[0], input[1]);
    let (mut l1, mut r1) = (0, k / 2 + 1);
    let (mut l2, mut r2) = (k / 2 + 1, k + 1);

    while l1 < r1 {
        let mid = l1 + (r1 - l1) / 2;
        if mid * (k - mid) <= v {
            l1 = mid + 1;
        } else {
            r1 = mid;
        }
    }

    while l2 < r2 {
        let mid = l2 + (r2 - l2) / 2;
        if mid * (k - mid) > v {
            l2 = mid + 1;
        } else {
            r2 = mid;
        }
    }

    print!("{} and {}", l2 - l1, solve_in_o1(k as f64, v as f64));
}

fn solve_in_o1(total_time: f64, target: f64) -> usize {
    let lower_bound = (total_time - (total_time * total_time - 4.0 * target).sqrt()) / 2.0;
    let upper_bound = (total_time + (total_time * total_time - 4.0 * target).sqrt()) / 2.0;
    upper_bound.floor() as usize - lower_bound.ceil() as usize + 1
}
