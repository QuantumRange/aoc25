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

    let mut alpha = 50i32;
    let mut code = 0;

    for x in instructions {
        let full_rotations = (x / 100).abs();
        let sub_rotations = x % 100;

        code += full_rotations;

        if alpha != 0
            // Positive overflow
            && ((sub_rotations > 0 && alpha >= 100 - sub_rotations)
            // Negative overflow
                || (sub_rotations < 0 && alpha <= sub_rotations.abs()))
        {
            code += 1;
        }

        alpha = (100 + alpha + sub_rotations) % 100;
    }

    println!("{}", code);
}
