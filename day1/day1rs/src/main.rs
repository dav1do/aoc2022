fn main() {
    a();
    b();
}

fn a() {
    let input = input_to_vec();
    println!("most cals: {}", input.last().unwrap());
}

fn b() {
    let input = input_to_vec();
    let well_stocked: i32 = input.into_iter().rev().take(3).sum();
    println!("top 3: {}", well_stocked);
}

fn input_to_vec() -> Vec<i32> {
    let mut input = include_str!("../../input.txt")
        .split("\n\n")
        .into_iter()
        .map(|e| {
            e.split("\n")
                .into_iter()
                .map(|c| c.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    input.sort();
    input
}
