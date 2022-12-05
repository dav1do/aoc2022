use std::collections::HashSet;

fn main() {
    a();
    b();
}

fn a() {
    let input = include_str!("./input.txt")
        .lines()
        .into_iter()
        .fold(0, |mut acc, l| {
            let val = l.split(",").collect::<Vec<&str>>();
            let a = parse_set(val[0]);
            let b = parse_set(val[1]);
            let contained = if a.len() < b.len() {
                b.is_superset(&a)
            } else if a.len() > b.len() {
                a.is_superset(&b)
            } else {
                a == b
            };
            if contained {
                acc += 1
            }
            acc
        });

    println!("{}", input);
}

fn b() {
    let input = include_str!("./input.txt")
        .lines()
        .into_iter()
        .fold(0, |mut acc, l| {
            let val = l.split(",").collect::<Vec<&str>>();
            let a = parse_set(val[0]);
            let b = parse_set(val[1]);
            if !a.is_disjoint(&b) {
                acc += 1
            }
            acc
        });

    println!("{}", input);
}

fn parse_set(val: &str) -> HashSet<i32> {
    let range = val
        .split("-")
        .into_iter()
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    HashSet::from_iter(range[0]..=range[1])
}
