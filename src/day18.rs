use std::fs;


fn part1(contents: &String) {
    let mut result = 1;
    let mut width = 1;
    for line in contents.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let dir = split[0];
        let steps = split[1].parse::<i32>().expect("Should be number");
        match dir {
            "L" => {
                width -= steps;
            },
            "R" => {
                width += steps;
                result += steps;
            },
            "U" => {
                result -= steps * (width - 1);
            },
            "D" => {
                result += steps * width;
            },
            _ => (),
        }
    }
    println!("{result}");
}

fn part2(contents: &String) {
    let mut result: i64 = 1;
    let mut width: i64 = 1;

    for line in contents.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let hex = split[2];
        let dir = hex[7..=7].parse::<i32>().expect("Should be number");
        let steps_hex = &hex[2..7];
        let mut steps: i64 = 0;
        for c in steps_hex.chars() {
            steps = steps * 16 + c.to_digit(16).expect("Should be number") as i64;
        }

        match dir {
            2 => { // L
                width -= steps;
            },
            0 => { // R
                width += steps;
                result += steps;
            },
            3 => { // U
                result -= steps * (width - 1);
            },
            1 => { // D
                result += steps * width;
            },
            _ => (),
        }
    }
    println!("{result}");    
}

pub fn main() {
    let contents = fs::read_to_string("input/day18.txt").expect("");

    part1(&contents);
    part2(&contents);
}