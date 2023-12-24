use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let map: Vec<Vec<char>> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let start = map[0].iter().position(|&c| c == '.').unwrap();
    let (m, n) = (map.len(), map[0].len());
    let visited = vec![vec![false; n]; m];

    println!("Part 1: {}", solve_part_1(0, start, &map, &visited));
    println!("Part 2: {}", solve_part_2(start, &map));
}

fn solve_part_1(i: usize, j: usize, map: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>) -> i32 {
    if i >= map.len() || j >= map[0].len() || map[i][j] == '#' || visited[i][j] {
        return i32::MIN;
    }
    if i == map.len() - 1 && map[i][j] == '.' {
        return 0;
    }

    let mut visited = visited.clone();
    visited[i][j] = true;

    1 + match map[i][j] {
        '.' => solve_part_1(i + 1, j, map, &visited)
            .max(solve_part_1(i, j + 1, map, &visited))
            .max(solve_part_1(i - 1, j, map, &visited))
            .max(solve_part_1(i, j - 1, map, &visited)),
        'v' => solve_part_1(i + 1, j, map, &visited),
        '>' => solve_part_1(i, j + 1, map, &visited),
        '^' => solve_part_1(i - 1, j, map, &visited),
        '<' => solve_part_1(i, j - 1, map, &visited),
        _ => unreachable!(),
    }
}

fn solve_part_2(start: usize, map: &Vec<Vec<char>>) -> i32 {
    let compressed = compress_grid(map);
    dfs(0, start, map.len() - 1, &HashSet::new(), &compressed)
}

fn dfs(
    i: usize,
    j: usize,
    end: usize,
    visited: &HashSet<(usize, usize)>,
    map: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> i32 {
    let mut visited = visited.clone();
    if !visited.insert((i, j)) {
        return i32::MIN;
    }
    if i == end {
        return 0;
    }

    map.get(&(i, j))
        .unwrap()
        .iter()
        .map(|&(x, y)| {
            dfs(x, y, end, &visited, map)
                + (i as i32 - x as i32).abs()
                + (j as i32 - y as i32).abs()
        })
        .max()
        .unwrap_or(-1)
}

fn compress_grid(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut compressed = HashMap::new();
    let (m, n) = (map.len(), map[0].len());

    let mut push = |(a, b), (c, d)| {
        compressed.entry((a, b)).or_insert(Vec::new()).push((c, d));
        compressed.entry((c, d)).or_insert(Vec::new()).push((a, b));
    };

    for i in 0..m {
        let mut k = 0;

        for j in 0..n {
            if map[i][j] == '#' {
                k = j + 1;
                continue;
            }
            if j == n - 1
                || (i + 1 < m && map[i + 1][j] != '#')
                || (i - 1 < m && map[i - 1][j] != '#')
            {
                if j > k {
                    push((i, j), (i, k));
                }
            }
        }
    }

    for j in 0..n {
        let mut k = 0;

        for i in 0..m {
            if map[i][j] == '#' {
                k = i + 1;
                continue;
            }
            if i == m - 1
                || (j + 1 < n && map[i][j + 1] != '#')
                || (j - 1 < n && map[i][j - 1] != '#')
            {
                if i > k {
                    push((i, j), (k, j));
                }
            }
        }
    }

    compressed
}
