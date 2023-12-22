use std::fs;

fn part1(contents: &String) {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut result = 0;
    for (i, line) in contents.lines().enumerate() {
        let split: Vec<&str> = line.split(&[':', ',', ';'][..]).collect();
        let mut valid = true;
        for turn in &split[1..] {
            let turn: Vec<&str> = turn.trim().split(' ').collect();
            let num = turn[0].parse::<i32>().expect("Should be integer");
            match turn[1] {
                "red" => if num > max_red {valid = false; break},
                "green" => if num > max_green {valid = false; break},
                "blue" => if num > max_blue {valid = false; break},
                _ => panic!{"Invalid colour"},
            }
        }
        if valid {result += i + 1;}
    }

    println!("{result}");
}

fn part2(contents: &String) {
    let mut result = 0;
    for line in contents.lines() {
        let split: Vec<&str> = line.split(&[':', ',', ';']).collect();
        let mut min = (-1, -1, -1); // r g b
        for turn in &split[1..] {
            let turn: Vec<&str> = turn.trim().split(' ').collect();
            let num = turn[0].parse::<i32>().expect("Should be integer");
            match turn[1] {
                "red" => if min.0 == -1 || num > min.0 {min.0 = num},
                "green" => if min.1 == -1 || num > min.1 {min.1 = num},
                "blue" => if min.2 == -1 || num > min.2 {min.2 = num},
                _ => panic!("Invalid colour"),
            }
        }
        result += min.0 * min.1 * min.2;
    }
    println!("{result}");
}

pub fn main() {
    let filepath = "input/day02.txt";

    let contents = fs::read_to_string(filepath).expect("Should have been able to read file");

    part1(&contents);
    part2(&contents);
}