use std::collections::HashSet;

fn main() {
    a();
    b();
}

fn char_points(r: &char) -> u32 {
    // ascii points A = 65, a = 97
    let val = if r.is_ascii_lowercase() {
        *r as u32 - 96
    } else {
        *r as u32 - 38
    };
    val
}

fn a() {
    let res = include_str!("./input.txt")
        .split("\n")
        .into_iter()
        .fold(0, |mut acc, l| {
            let a: HashSet<char> = HashSet::from_iter(l[..(l.len() / 2)].chars());
            let b = HashSet::from_iter(l[(l.len() / 2)..].chars());
            let res = a.intersection(&b);
            acc += res.into_iter().map(|r| char_points(r)).sum::<u32>();
            acc
        });

    println!("{}", res);
}

fn b() {
    let input = include_str!("./input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut acc = 0;
    for (idx, val) in input.iter().enumerate().step_by(3) {
        let mut intersection: HashSet<char> = HashSet::from_iter(val.chars());
        let others: Vec<HashSet<char>> = vec![
            HashSet::from_iter(input[idx + 1].chars()),
            HashSet::from_iter(input[idx + 2].chars()),
        ];
        for other in others {
            intersection.retain(|e| other.contains(e));
        }
        acc += intersection.iter().map(|a| char_points(a)).sum::<u32>();
    }

    println!("{}", acc);
}
