use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut cards: Vec<(String, usize)> = stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let card = line.split_whitespace().collect::<Vec<_>>();
            (card[0].to_owned(), card[1].parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    cards.sort_unstable_by(|(a, _), (b, _)| {
        let type_of_a = get_type_of_card(a);
        let type_of_b = get_type_of_card(b);
        match type_of_a.cmp(&type_of_b) {
            Equal => compare_hands_strength(a, b),
            x => x,
        }
    });

    let ans = cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, x))| acc + x * (i + 1));

    println!("{ans}");
}

fn get_type_of_card(card: &String) -> usize {
    let mut map = [0; 128];
    card.as_bytes().iter().for_each(|&ch| map[ch as usize] += 1);

    let mut map = map
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i as u8 as char))
        .filter(|&(x, _)| x > 0)
        .collect::<Vec<_>>();

    let idx = map
        .iter()
        .position(|&(_, ch)| ch == 'J')
        .and_then(|i| Some(map.remove(i)));

    map.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
        Equal => get_hand_strength(a.1).cmp(&get_hand_strength(b.1)),
        x => x,
    });

    if let Some((i, _)) = idx {
        let (j, ch) = map.pop().unwrap_or((0, 'A'));
        map.push((i + j, ch));
    }

    match map.last().unwrap().0 {
        x if x > 3 => x + 5,
        x => 7 + x - map.len(),
    }
}

fn compare_hands_strength(a: &String, b: &String) -> Ordering {
    for (ch_a, ch_b) in a.chars().zip(b.chars()) {
        match get_hand_strength(ch_a).cmp(&get_hand_strength(ch_b)) {
            Equal => continue,
            x => return x,
        }
    }
    unreachable!("Invalid Input");
}

fn get_hand_strength(ch: char) -> usize {
    match ch {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        'J' => 1,
        _ => ch.to_digit(10).unwrap() as usize,
    }
}
