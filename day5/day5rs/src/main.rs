use std::collections::BTreeMap;

fn main() {
    let (stacks, moves) = include_str!("./input.txt").split_once("\n\n").unwrap();

    let mut input_rows: BTreeMap<usize, Vec<char>> = BTreeMap::default();
    for (i, stack) in stacks.split("\n").into_iter().enumerate() {
        input_rows.insert(i + 1, stack.chars().collect::<Vec<char>>());
    }
    // last entry is stack number, so index i of every other vec is the items in the stack
    let mut piles: BTreeMap<u32, Vec<char>> = BTreeMap::default();

    for (i, v) in input_rows
        .get(&input_rows.len())
        .as_ref()
        .unwrap()
        .iter()
        .enumerate()
    {
        if v.is_ascii_digit() {
            let mut stack = vec![' '; input_rows.len() - 1];
            for (key, vec) in input_rows.iter() {
                if *key != input_rows.len() {
                    stack.insert(*key, vec[i]);
                }
            }
            piles.insert(v.to_digit(10).unwrap(), stack);
        }
    }
    let mut good_piles_a: BTreeMap<u32, Vec<char>> = BTreeMap::default();
    for (i, v) in piles.into_iter() {
        let mut v = v
            .into_iter()
            .filter(|v| !v.is_whitespace())
            .collect::<Vec<char>>();
        v.reverse();
        good_piles_a.insert(i, v);
    }

    let mut good_piles_b = good_piles_a.clone();

    for val in moves.split("\n").into_iter() {
        let split = val.split(" ").collect::<Vec<&str>>();
        let cnt: u32 = split[1].parse().unwrap();
        let from: u32 = split[3].parse().unwrap();
        let to: u32 = split[5].parse().unwrap();
        let removing_a = good_piles_a.get_mut(&from).unwrap();
        let removing_b = good_piles_b.get_mut(&from).unwrap();
        let mut updated_a = vec![];
        let mut updated_b = vec![];
        // could do B by slicing the array and moving it rather than the pop/build/reverse, but it's easy and don't need to refactor
        for _ in 0..cnt {
            let moved = removing_a.pop().unwrap();
            updated_a.push(moved);
            let moved = removing_b.pop().unwrap();
            updated_b.push(moved);
        }
        let adding_a = good_piles_a.get_mut(&to).unwrap();
        let adding_b = good_piles_b.get_mut(&to).unwrap();
        for v in updated_a.into_iter() {
            adding_a.push(v)
        }
        for v in updated_b.into_iter().rev() {
            adding_b.push(v);
        }
    }
    for (_pile, v) in good_piles_a.into_iter() {
        print!("{}", v.get(v.len() - 1).unwrap())
    }
    println!();
    for (_pile, v) in good_piles_b.into_iter() {
        print!("{}", v.get(v.len() - 1).unwrap())
    }
    println!();
}
