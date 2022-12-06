use std::collections::HashSet;

fn main() {
    a();
    b();
}

fn a() {
    doit(4);
}

fn b() {
    doit(14);
}

fn doit(chars: usize) {
    let input = include_str!("./input.txt").chars().collect::<Vec<char>>();
    let mut pos = 0;
    for (i, _signal) in input.iter().enumerate() {
        if i + chars - 1 < input.len() {
            let set: HashSet<&char> = HashSet::from_iter(input[i..i + chars].into_iter());
            if set.len() == chars {
                pos = i + chars; // want end of marker
                break;
            }
        }
    }

    println!("{}", pos);
}
