use std::{fs, collections::VecDeque};


fn part1(contents: &String) {
    let mut arr: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        arr.push(row);
    }

    println!("{}", do_part_1(0, 0, 0, &arr));
}

fn do_part_1(i: usize, j: usize, dir: i32, arr: &Vec<Vec<char>>) -> i32 {
    let mut light: Vec<Vec<i32>> = Vec::new();
    // Light: one hot encoding of 0-15 corresponding to 4 directions
    for _ in 0..arr.len() {
        let mut light_row: Vec<i32> = Vec::new();
        for _ in 0..arr[0].len() {
            light_row.push(0);
        }
        light.push(light_row);
    }

    let max_x = (arr.len()-1) as i32;
    let max_y = (arr[0].len()-1) as i32;

    let mut beams: VecDeque<(i32, i32, i32)> = VecDeque::new();
    beams.push_back((i as i32, j as i32, dir));

    let mut result = 0;

    while beams.len() > 0 {
        let beam = beams.pop_front().unwrap();
        let i = beam.0 as usize;
        let j = beam.1 as usize;
        let x = beam.0;
        let y = beam.1;

        if x < 0 || x > max_x || y < 0 || y > max_y || (light[i][j] >> beam.2) % 2 == 1{
            continue;
        }

        if light[i][j] == 0 {
            result += 1;
        }

        match beam.2 {
            0 => { // right, enc=1
                match arr[i][j] {
                    '.' | '-' => {
                        light[i][j] |= 5;
                        beams.push_back((x, y+1, 0));
                    },
                    '/' => {
                        light[i][j] |= 3;
                        beams.push_back((x-1, y, 3));
                    },
                    '\\' => {
                        light[i][j] |= 9;
                        beams.push_back((x+1, y, 1));
                    },
                    '|' => {
                        light[i][j] |= 11;
                        beams.push_back((x-1, y, 3));
                        beams.push_back((x+1, y, 1));
                    },
                    _ => (),
                }
            },
            1 => { // down, enc=2
                match arr[i][j] {
                    '.' | '|' => {
                        light[i][j] |= 10;
                        beams.push_back((x+1, y, 1));
                    },
                    '/' => {
                        light[i][j] |= 3;
                        beams.push_back((x, y-1, 2));
                    },
                    '\\' => {
                        light[i][j] |= 6;
                        beams.push_back((x, y+1, 0));
                    },
                    '-' => {
                        light[i][j] |= 7;
                        beams.push_back((x, y-1, 2));
                        beams.push_back((x, y+1, 0));
                    },
                    _ => (),
                }
            },
            2 => { // left, enc=4
                match arr[i][j] {
                    '.' | '-' => {
                        light[i][j] |= 5;
                        beams.push_back((x, y-1, 2));
                    },
                    '/' => {
                        light[i][j] |= 12;
                        beams.push_back((x+1, y, 1));
                    },
                    '\\' => {
                        light[i][j] |= 6;
                        beams.push_back((x-1, y, 3));
                    },
                    '|' => {
                        light[i][j] |= 14;
                        beams.push_back((x-1, y, 3));
                        beams.push_back((x+1, y, 1));
                    },
                    _ => (),
                }
            },
            3 => { // up, enc=8
                match arr[i][j] {
                    '.' | '|' => {
                        light[i][j] |= 10;
                        beams.push_back((x-1, y, 3));
                    },
                    '/' => {
                        light[i][j] |= 12;
                        beams.push_back((x, y+1, 0));
                    },
                    '\\' => {
                        light[i][j] |= 9;
                        beams.push_back((x, y-1, 2));
                    },
                    '-' => {
                        light[i][j] |= 13;
                        beams.push_back((x, y-1, 2));
                        beams.push_back((x, y+1, 0));
                    },
                    _ => (),
                }
            },
            _ => (),
        }
        // println!("{} {} ({}, {})", match beam.2 {
        //     0 => ">",
        //     1 => "v",
        //     2 => "<",
        //     3 => "^",
        //     _ => panic!("Invalid"),
        // }, arr[i][j], i, j);
    }

    result
}

fn part2(contents: &String) {
    let mut arr: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        arr.push(row);
    }

    let max_i = arr.len()-1;
    let max_j = arr[0].len()-1;

    let mut result = 0;
    for i in 0..=max_i {
        let mut temp = do_part_1(i, 0, 0, &arr);
        if temp > result {result = temp;}

        temp = do_part_1(i, max_j, 2, &arr);
        if temp > result {result = temp;}
    }
    for j in 0..=max_j {
        let mut temp = do_part_1(0, j, 1, &arr);
        if temp > result {result = temp;}

        temp = do_part_1(max_i, j, 3, &arr);
        if temp > result {result = temp;}
    }
    println!("{result}")
}

pub fn main() {
    let contents = fs::read_to_string("input/day16.txt").expect("");

    part1(&contents);
    part2(&contents);
}