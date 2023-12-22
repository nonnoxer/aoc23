use std::fs;


fn row_eq(arr: &Vec<Vec<i32>>, a: usize, b: usize) -> bool {
    for i in 0..arr[0].len() {
        if arr[a][i] != arr[b][i] {
            return false;
        }
    }
    true
}

fn col_eq(arr: &Vec<Vec<i32>>, a: usize, b: usize) -> bool {
    for i in 0..arr.len() {
        if arr[i][a] != arr[i][b] {
            return false;
        }
    }
    true
}

fn solve(block: &str) -> usize {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for line in block.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid character"),
            });
        }
        arr.push(row);
    }

    'col: for j in 0..arr[0].len()-1 {
        if col_eq(&arr, j, j+1) {
            let mut k = 0;
            while j >= k && j+1+k < arr[0].len() {
                if !col_eq(&arr, j-k, j+1+k) {
                    continue 'col;
                }
                k += 1;
            }
            return j + 1;
        }
    }
    'row: for i in 0..arr.len() - 1 {
        if row_eq(&arr, i, i+1) {
            let mut k = 0;
            while i >= k && i+1+k < arr.len() {
                if !row_eq(&arr, i-k, i+1+k) {
                    continue 'row;
                }
                k += 1;
            }
            return (i + 1) * 100;
        }
    }
    panic!("no solution");
}

fn part1(contents: &String) {
    let blocks: Vec<&str> = contents.split("\n\n").collect();

    let mut result = 0;
    for block in blocks {
        result += solve(block);
    }

    println!("{result}");
}

fn row_diff(arr: &Vec<Vec<i32>>, a: usize, b: usize) -> i32 {
    let mut sum = 0;
    for j in 0..arr[0].len() {
        if arr[a][j] != arr[b][j] {
            sum += 1;
        }
    }
    sum
}

fn col_diff(arr: &Vec<Vec<i32>>, a: usize, b: usize) -> i32 {
    let mut sum = 0;
    for i in 0..arr.len() {
        if arr[i][a] != arr[i][b] {
            sum += 1;
        }
    }
    sum
}

fn solve2(block: &str) -> usize {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for line in block.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid character"),
            });
        }
        arr.push(row);
    }

    'col: for j in 0..arr[0].len()-1 {
        let mut balance = 1 - col_diff(&arr, j, j+1);
        if balance >= 0 {
            let mut k = 1;
            while j >= k && j+1+k < arr[0].len() {
                balance -= col_diff(&arr, j-k, j+1+k);
                if balance < 0 {
                    continue 'col;
                }
                k += 1;
            }
            if balance == 0 {
                return j + 1;
            }
        }
    }
    'row: for i in 0..arr.len() - 1 {
        let mut balance = 1 - row_diff(&arr, i, i+1);
        if balance >= 0 {
            let mut k = 1;
            while i >= k && i+1+k < arr.len() {
                balance -= row_diff(&arr, i-k, i+1+k);
                if balance < 0 {
                    continue 'row;
                }
                k += 1;
            }
            if balance == 0 {
                return (i + 1) * 100;
            }
        }
    }
    panic!("no solution");
}

fn part2(contents: &String) {
    let blocks: Vec<&str> = contents.split("\n\n").collect();

    let mut result = 0;
    for block in blocks {
        result += solve2(block);
    }

    println!("{result}");
}

pub fn main() {
   let contents = fs::read_to_string("input/day13.txt").expect("");

    part1(&contents);
    part2(&contents);
}