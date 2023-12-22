use std::fs;

fn part1(contents: &String) {
    let mut result = 0;
    for line in contents.lines() {
        let line = line.replace("  ", " ");
        let splits: Vec<&str> = line.split(&[':', '|']).collect();
        let my_nums = splits[1].trim().split(' ');
        let winning_nums: Vec<&str> = splits[2].trim().split(' ').collect();
        let mut score = 0;
        for num in my_nums {
            match winning_nums.iter().position(|&r| r.eq(num)) {
                Some(_) => score = match score {
                    0 => 1,
                    other => other * 2,
                },
                None => (),
            }
        }
        result += score;
    }
    println!("{result}");
}

fn part2(contents: &String) {
    let mut result = 0;
    let length = 213;
    let mut multiplier = vec![];
    for _ in 0..length {
        multiplier.push(1);
    }

    for (i, line) in contents.lines().enumerate() {
        let line = line.replace("  ", " ");
        let splits: Vec<&str> = line.split(&[':', '|']).collect();
        let my_nums = splits[1].trim().split(' ');
        let winning_nums: Vec<&str> = splits[2].trim().split(' ').collect();
        let mut score = 0;
        for num in my_nums {
            match winning_nums.iter().position(|&r| r.eq(num)) {
                Some(_) => score += 1,
                None => (),
            }
        }
        for j in 1..=score {
            if i < length {
                multiplier[i + j] += multiplier[i];
            }
        }
        result += multiplier[i];
    }
    println!("{result}");
}

pub fn main() {
    let filepath = "input/day04.txt";
    let contents = fs::read_to_string(filepath).expect("Should have been able to read file");

    part1(&contents);
    part2(&contents);
}