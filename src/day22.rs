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

    let mut score: u64 = 0;

    for (from, to) in ids {
        for x in from..=to {
            let x_str = x.to_string();

            for rep in 2..=(x_str.len()) {
                if x_str.len() % rep != 0 {
                    continue;
                }

                let com_size = x_str.len() / rep;
                let first = &x_str[0..com_size];

                if (1..=(rep - 1)).all(|offset| {
                    &x_str[com_size * offset..com_size * (offset + 1)] == first
                }) {
                    score += x;
                    break;
                }
            }
        }
    }
}