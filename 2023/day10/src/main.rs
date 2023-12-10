use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let (mut si, mut sj) = (0, 0);
    let mut tiles = Vec::new();

    for line in stdin().lock().lines() {
        let line = line.unwrap();

        tiles.push(
            line.chars()
                .enumerate()
                .map(|(i, ch)| {
                    if ch == 'S' {
                        (si, sj) = (tiles.len(), i)
                    };
                    ch
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut part1 = 0;
    let mut adj = tiles.clone();

    if ['|', 'J', 'L'].contains(&adj[si + 1][sj]) {
        queue.push((si + 1, sj));
    } else if ['-', 'J', 'F'].contains(&adj[si][sj + 1]) {
        queue.push((si, sj + 1));
    } else if sj > 0 && ['-', 'L', 'F'].contains(&adj[si][sj.wrapping_sub(1)]) {
        queue.push((si, sj.wrapping_sub(1)));
    } else if si > 0 && ['|', 'J', 'F'].contains(&adj[si.wrapping_sub(1)][sj]) {
        queue.push((si.wrapping_sub(1), sj));
    }

    while !queue.is_empty() {
        let (i, j) = queue.pop().unwrap();
        if adj[i][j] == '.' || adj[i][j] == '#' {
            continue;
        };
        let curr = adj[i][j];
        adj[i][j] = '#';

        match curr {
            '|' => {
                queue.push((i + 1, j));
                queue.push((i.wrapping_sub(1), j));
            }
            '-' => {
                queue.push((i, j + 1));
                queue.push((i, j.wrapping_sub(1)));
            }
            'L' => {
                queue.push((i, j + 1));
                queue.push((i.wrapping_sub(1), j));
            }
            'J' => {
                queue.push((i, j.wrapping_sub(1)));
                queue.push((i.wrapping_sub(1), j));
            }
            '7' => {
                queue.push((i + 1, j));
                queue.push((i, j.wrapping_sub(1)));
            }
            'F' => {
                queue.push((i + 1, j));
                queue.push((i, j + 1));
            }
            _ => (),
        }

        part1 += 1;
    }

    let mut part2 = 0;
    let mut new_adj = adj.clone();

    for i in 0..adj.len() {
        for j in 0..adj[0].len() {
            if adj[i][j] != '#' {
                let mut count = 0;

                for k in j + 1..adj[0].len() {
                    if adj[i][k] == '#' && ['F', '7', '|'].contains(&tiles[i][k]) {
                        count += 1;
                    }
                }

                if count % 2 == 1 {
                    part2 += dfs(i, j, &mut new_adj);
                } else {
                    dfs(i, j, &mut new_adj);
                }
            }
        }
    }

    println!("Part1: {}", (part1 + 1) / 2);
    println!("Part 2: {part2}");
}

fn dfs(i: usize, j: usize, adj: &mut Vec<Vec<char>>) -> i32 {
    if i >= adj.len() || j >= adj[0].len() || adj[i][j] == '#' {
        return 0;
    };
    adj[i][j] = '#';
    1 + dfs(i + 1, j, adj)
        + dfs(i, j + 1, adj)
        + dfs(i, j.wrapping_sub(1), adj)
        + dfs(i.wrapping_sub(1), j, adj)
}
