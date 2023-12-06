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

    print!("{}", l2 - l1);
}
