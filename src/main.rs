use std::fs;

fn main() {
    let instructions: Vec<i32> = fs::read_to_string("day1_1.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| match line.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!("Invalid direction"),
            } * line[1..].parse::<i32>().expect("Invalid number")
        )
        .collect::<Vec<_>>();

    let mut dial = 50i32;
    let mut solution = 0;

    for x in instructions {
        dial += x;
        dial %= 100;

        if dial == 0 {
            solution += 1;
        }
    }

    println!("{}", solution);
}
