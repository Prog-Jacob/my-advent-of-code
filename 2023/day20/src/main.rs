use std::collections::HashMap;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut types: HashMap<String, String> = HashMap::new();
    let mut to: HashMap<String, Vec<String>> = HashMap::new();
    let mut from: HashMap<String, Vec<String>> = HashMap::new();

    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let mut module = line.split(" -> ");

        let mut src = module.next().unwrap();
        let dists: Vec<_> = module.next().unwrap().split(", ").collect();

        if src.starts_with(['%', '&']) {
            let t = &src[0..1];
            src = &src[1..];
            types.insert(src.to_string(), t.to_string());

            for dist in &dists {
                from.entry(dist.to_string())
                    .or_insert(Vec::new())
                    .push(src.to_string());
            }
        }

        for dist in &dists {
            to.entry(src.to_string())
                .or_insert(Vec::new())
                .push(dist.to_string());
        }
    }

    solve(&types, &to, &from);
}

fn solve(
    types: &HashMap<String, String>,
    to: &HashMap<String, Vec<String>>,
    from: &HashMap<String, Vec<String>>,
) {
    let (mut i, mut low, mut high) = (1, 0, 0);
    let mut vals: HashMap<String, bool> = to
        .keys()
        .chain(from.keys())
        .map(|k| (k.to_string(), false))
        .collect();
    let mut backdoor: HashMap<_, _> = ["tx", "dd", "nz", "ph"]
        .iter()
        .map(|s| (s.to_string(), 0))
        .collect();

    // Consider using a HashSet with Stack to find cycles.
    while i <= 1000 || backdoor.values().any(|&v| v == 0) {
        let (l, h) = dfs("broadcaster", types, &mut vals, to, from, &mut backdoor, i);
        if i <= 1000 {
            low += l + 1;
            high += h;
        }
        i += 1;
    }

    println!("Part 1: {}", low * high);
    println!(
        "Part 2: {}",
        backdoor.values().fold(1, |acc, &v| lcm(acc, v))
    );
}

fn dfs(
    node: &str,
    types: &HashMap<String, String>,
    vals: &mut HashMap<String, bool>,
    to: &HashMap<String, Vec<String>>,
    from: &HashMap<String, Vec<String>>,
    backdoor: &mut HashMap<String, usize>,
    presses: usize,
) -> (usize, usize) {
    let mut low = 0;
    let mut high = 0;
    let mut forward = *vals.get(node).unwrap();

    if let Some(t) = types.get(node) {
        if t == "&" {
            forward = from
                .get(node)
                .unwrap()
                .iter()
                .any(|n| !*vals.get(n).unwrap());
            vals.insert(node.to_string(), forward);
        }
    } else if node != "broadcaster" {
        return (0, 0);
    };

    for dist in to.get(node).unwrap() {
        let val = vals.get_mut(dist).unwrap();

        if let Some(t) = types.get(dist) {
            if t == "%" {
                *val = !(*val ^ forward);
            }
        } else {
            *val = forward;
        }
    }

    for dist in to.get(node).unwrap() {
        if forward && types.get(dist).unwrap_or(&String::new()) == "%" {
            continue;
        } else if !forward && backdoor.contains_key(dist) {
            *backdoor.get_mut(dist).unwrap() = presses;
        }

        let (l, h) = dfs(dist, types, vals, to, from, backdoor, presses);
        high += h;
        low += l;
    }

    if forward {
        (low, to.get(node).unwrap().len() + high)
    } else {
        (to.get(node).unwrap().len() + low, high)
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b != 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let (a, b) = (a.max(b), a.min(b));
    b * (a / gcd(a, b))
}
