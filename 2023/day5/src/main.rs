use std::collections::HashSet;
use std::io::prelude::*;
use std::io::stdin;
use std::num::Wrapping;

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let input = s.split("\r\n\r\n");
    let mut queue: Vec<(Wrapping<u32>, Wrapping<u32>)> = Vec::new();

    for (i, block) in input.enumerate() {
        let lines = block.split(if i == 0 { ":" } else { "\r\n" });
        let mut visisted: HashSet<(Wrapping<u32>, Wrapping<u32>)> = HashSet::new();
        let mut ans = HashSet::new();
        let mut ranges = Vec::new();

        for line in lines.skip(1) {
            if line.is_empty() {
                break;
            };
            let range = line
                .split_whitespace()
                .map(|x| Wrapping(x.parse::<u32>().unwrap()))
                .collect::<Vec<_>>();
            if i == 0 {
                let n = range.len() / 2;
                ans = (0..n)
                    .map(|j| (range[2 * j], range[2 * j] + range[2 * j + 1]))
                    .collect();
            } else {
                ranges.push(range);
            }
        }

        while !queue.is_empty() {
            let (a, b) = queue.pop().unwrap();
            if visisted.contains(&(a, b)) {
                ans.insert((a, b));
                continue;
            }
            visisted.insert((a, b));
            let mut flag = true;
            for range in &ranges {
                let start = a.max(range[1]);
                let end = b.min(range[1] + range[2]);

                if start < end {
                    let first = range[0] + start - range[1];
                    ans.insert((first, first + end - start));
                } else {
                    continue;
                };
                flag = false;
                if a < start {
                    let next = (a, start.min(b));
                    queue.push(next);
                }
                if b > end {
                    let next = (end.max(a), b);
                    queue.push(next);
                }
            }
            if flag {
                ans.insert((a, b));
            }
        }
        queue = ans.into_iter().collect();
    }
    println!("{}", queue.into_iter().min().unwrap().0);
}
