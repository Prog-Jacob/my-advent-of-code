use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut ans = 0;

    for line in stdin().lock().lines() {
        ans += calc_next(
            &line
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }

    println!("{ans}");
}

fn calc_next(nums: &Vec<i32>) -> i32 {
    if nums.last().unwrap() == &0 {
        return 0;
    }
    return nums[0] - calc_next(&nums.windows(2).map(|x| x[1] - x[0]).collect());
}
