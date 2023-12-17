use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut ans = 0;
    let mut last = 0;
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in stdin().lock().lines() {
        let s = line.unwrap();
        let mut is_first = true;

        for (i, ch) in s.chars().enumerate() {
            if ch.is_digit(10) {
                last = ch.to_digit(10).unwrap();
                if is_first {
                    ans += last * 10;
                    is_first = false;
                }
                continue;
            };
            for (j, num) in nums.iter().enumerate() {
                if i + num.len() <= s.len() && &s[i..i + num.len()] == *num {
                    last = (j + 1) as u32;
                    if is_first {
                        ans += last * 10;
                        is_first = false;
                    }
                    break;
                }
            }
        }
        ans += last;
    }

    println!("{ans}");
}
