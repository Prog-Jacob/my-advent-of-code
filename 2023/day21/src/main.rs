use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    let garden = stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(j, ch)| {
                    if ch == 'S' {
                        queue.push_back((i, j, 0));
                        true
                    } else {
                        ch == '.'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    const DIRS: [(usize, usize); 4] = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
    let (m, n) = (garden.len(), garden[0].len());
    let mut visited = vec![vec![-1; n]; n];

    const STEPS: usize = 26501365;
    let p = STEPS % 2;

    while let Some((i, j, mut step)) = queue.pop_front() {
        if visited[i][j] != -1 {
            continue;
        }
        visited[i][j] = step;
        step += 1;

        for &(di, dj) in &DIRS {
            let (x, y) = (i + di, j + dj);
            if x < m && y < n && garden[x][y] && visited[x][y] == -1 {
                queue.push_back((x, y, step));
            }
        }
    }

    let values = visited
        .iter()
        .flatten()
        .filter(|&&x| x != -1)
        .map(|&x| x as usize)
        .collect::<Vec<_>>();

    let copies = (STEPS - n / 2) / n;
    let odd = values.iter().filter(|&&x| x % 2 == p).count();
    let even = values.iter().filter(|&&x| x % 2 != p).count();
    let o_corner = values.iter().filter(|&&x| x % 2 == p && x > n / 2).count();
    let e_corner = values.iter().filter(|&&x| x % 2 != p && x > n / 2).count();

    let ans = (copies + 1).pow(2) * odd + copies.pow(2) * even - (copies + 1) * o_corner
        + copies * e_corner;
    println!("{}", ans);
}
