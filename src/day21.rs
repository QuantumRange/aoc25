use std::fs;

fn main() {
    let contents = fs::read_to_string("res/day21.txt").expect("Puzzle input missing");

    let ids = contents
        .split(",")
        .map(|range_str| {
            let mut range = range_str.split('-').map(|e| e.parse::<u64>().unwrap());

            let start = range.next().unwrap();
            let end = range.next().unwrap();

            (start, end)
        })
        .collect::<Vec<_>>();

    let mut score = 0u64;

    for (from, to) in ids {
        for x in from..=to {
            let str = x.to_string();

            if str.len() % 2 == 0 && str[0..str.len() / 2] == str[str.len() / 2..] {
                score += x
            }
        }
    }

    println!("{score}");
}