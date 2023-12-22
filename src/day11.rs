use std::fs;

fn dist(c1: (usize, usize), c2: (usize, usize)) -> usize {
    let mut distance = 0;
    if c1.0 >= c2.0 {distance += c1.0 - c2.0;}
    else {distance += c2.0 - c1.0};
    if c1.1 >= c2.1 {distance += c1.1 - c2.1;}
    else {distance += c2.1 - c1.1;}
    distance
}

fn part1(contents: &String) {
    let mut arr: Vec<Vec<char>> = Vec::new();
    let mut gals: Vec<(usize, usize)> = Vec::new();
    let mut cols_add: Vec<usize> = Vec::new();
    let mut rows_add: Vec<usize> = Vec::new();
    for (i, line) in contents.lines().into_iter().enumerate() {
        rows_add.push(1);
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().into_iter().enumerate() {
            if i == 0 {cols_add.push(1);}
            row.push(c);
            if c == '#' {
                rows_add[i] = 0;
                cols_add[j] = 0;
                gals.push((i, j));
            }
        }
        arr.push(row);
    }
    let mut run_sum = 0;
    for j in 0..cols_add.len() {
        cols_add[j] += run_sum;
        run_sum = cols_add[j];
    }
    run_sum = 0;
    for i in 0..rows_add.len() {
        rows_add[i] += run_sum;
        run_sum = rows_add[i];
    }

    for i in 0..gals.len() {
        gals[i].0 += rows_add[gals[i].0];
        gals[i].1 += cols_add[gals[i].1];
    }

    let mut result = 0;
    for i in 0..gals.len()-1 {
        for j in i+1..gals.len() {
            result += dist(gals[i], gals[j]);
        }
    }
    println!("{result}");
}

fn part2(contents: &String) {
    let mut arr: Vec<Vec<char>> = Vec::new();
    let mut gals: Vec<(usize, usize)> = Vec::new();
    let mut cols_add: Vec<usize> = Vec::new();
    let mut rows_add: Vec<usize> = Vec::new();
    for (i, line) in contents.lines().into_iter().enumerate() {
        rows_add.push(999999);
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().into_iter().enumerate() {
            if i == 0 {cols_add.push(999999);}
            row.push(c);
            if c == '#' {
                rows_add[i] = 0;
                cols_add[j] = 0;
                gals.push((i, j));
            }
        }
        arr.push(row);
    }
    let mut run_sum = 0;
    for j in 0..cols_add.len() {
        cols_add[j] += run_sum;
        run_sum = cols_add[j];
    }
    run_sum = 0;
    for i in 0..rows_add.len() {
        rows_add[i] += run_sum;
        run_sum = rows_add[i];
    }

    for i in 0..gals.len() {
        gals[i].0 += rows_add[gals[i].0];
        gals[i].1 += cols_add[gals[i].1];
    }

    let mut result = 0;
    for i in 0..gals.len()-1 {
        for j in i+1..gals.len() {
            result += dist(gals[i], gals[j]);
        }
    }
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day11.txt").expect("");
    part1(&contents);
    part2(&contents);
}