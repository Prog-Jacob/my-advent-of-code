use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::stdin;
use std::time::Instant;

fn main() {
    let mut bricks: Vec<Vec<_>> = stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let brick: Vec<_> = line.split('~').collect();
            let s = brick[0]
                .split(',')
                .rev()
                .map(|x| x.parse::<usize>().unwrap());
            let e = brick[1]
                .split(',')
                .rev()
                .map(|x| x.parse::<usize>().unwrap());
            s.zip(e).collect()
        })
        .collect();
    bricks.sort_unstable();

    let start = Instant::now();

    let (part1, part2) = solve(&bricks);
    println!("Time: {:?}", start.elapsed());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn position_bricks(bricks: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<(usize, usize)>> {
    let mut rested_bricks: Vec<Vec<(usize, usize)>> = Vec::new();

    for brick in bricks {
        let height = rested_bricks
            .iter()
            .filter(|rested_brick| is_overlapping(brick, rested_brick))
            .map(|rested_brick| rested_brick[0].1)
            .max()
            .unwrap_or(0)
            + 1;

        rested_bricks.push(vec![
            (height, height + brick[0].1 - brick[0].0),
            brick[1],
            brick[2],
        ]);
    }

    rested_bricks
}

fn get_num_of_supporters(bricks: &Vec<Vec<(usize, usize)>>) -> HashMap<Vec<(usize, usize)>, usize> {
    bricks
        .iter()
        .enumerate()
        .rev()
        .map(|(i, brick)| {
            let a_start = brick[0].0;

            (
                brick.clone(),
                bricks
                    .iter()
                    .take(i)
                    .filter(|candidate| {
                        let b_end = candidate[0].1;
                        a_start == b_end + 1 && is_overlapping(brick, candidate)
                    })
                    .count(),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn get_dependencies(
    bricks: &Vec<Vec<(usize, usize)>>,
) -> HashMap<&Vec<(usize, usize)>, Vec<&Vec<(usize, usize)>>> {
    let mut supports = HashMap::new();

    bricks.iter().enumerate().rev().for_each(|(i, brick)| {
        let a_end = brick[0].1;

        let dependencies: Vec<_> = bricks
            .iter()
            .skip(i + 1)
            .filter(|candidate| {
                let b_start = candidate[0].0;
                a_end == b_start - 1 && is_overlapping(brick, candidate)
            })
            .collect();

        supports.insert(brick, dependencies);
    });

    supports
}

fn solve(bricks: &Vec<Vec<(usize, usize)>>) -> (usize, usize) {
    let bricks = position_bricks(bricks);
    let supports = get_dependencies(&bricks);
    let depends_on = get_num_of_supporters(&bricks);
    let (mut safe, mut not_safe) = (0, 0);

    bricks.iter().for_each(|brick| {
        let mut queue = VecDeque::from(supports[brick].clone());
        let mut depends_on = depends_on.clone();
        let mut current = 0;

        while let Some(candidate) = queue.pop_front() {
            *depends_on.get_mut(candidate).unwrap() -= 1;
            if depends_on[candidate] != 0 {
                continue;
            }
            current += 1;

            for brick in supports[&candidate].iter() {
                queue.push_back(brick);
            }
        }

        not_safe += current;
        if current == 0 {
            safe += 1;
        }
    });

    (safe, not_safe)
}

fn is_overlapping(a: &Vec<(usize, usize)>, b: &Vec<(usize, usize)>) -> bool {
    let (a_left, a_right) = a[1];
    let (a_bottom, a_top) = a[2];
    let (b_left, b_right) = b[1];
    let (b_bottom, b_top) = b[2];
    a_left <= b_right && a_right >= b_left && a_top >= b_bottom && a_bottom <= b_top
}
