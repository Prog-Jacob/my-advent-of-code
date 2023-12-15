use std::collections::HashMap;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let input = input.replace(&['\n', '\r'], "");
    let mut boxes: HashMap<usize, Vec<(&str, &str)>> = HashMap::new();

    input.split(',').for_each(|s| {
        let mut lense = s.split(&['-', '=']);
        let label = lense.next().unwrap();

        if s.ends_with('-') {
            if let Some(v) = boxes.get_mut(&hash(label)) {
                v.retain(|(l, _)| l != &label);
            }
        } else {
            let focus = lense.next().unwrap();

            if let Some(v) = boxes.get_mut(&hash(label)) {
                if let Some(pos) = v.iter().position(|(l, _)| l == &label) {
                    v[pos] = (label, focus);
                } else {
                    v.push((label, focus));
                }
            } else {
                boxes.insert(hash(label), vec![(label, focus)]);
            }
        }
    });

    let ans = boxes.iter().fold(0, |out, (label, lenses)| {
        out + (label + 1)
            * lenses.iter().enumerate().fold(0, |inner, (i, (_, focus))| {
                inner + (i + 1) * focus.parse::<usize>().unwrap()
            })
    });

    println!("{ans}");
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |hash, &ch| (hash + ch as usize) * 17 % 256)
}
