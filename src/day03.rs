use std::fs;

fn surround_symbol(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    fn is_symbol(c: char) -> bool {
        !c.is_digit(10) && c != '.'
    }

    let max_i = arr.len()-1;
    let max_j = arr[0].len()-1;
    (i > 0 && j > 0 && is_symbol(arr[i-1][j-1])) ||
    (i > 0 && is_symbol(arr[i-1][j])) ||
    (i > 0 && j < max_j && is_symbol(arr[i-1][j+1])) ||
    (j > 0 && is_symbol(arr[i][j-1])) ||
    (j < max_j && is_symbol(arr[i][j+1])) ||
    (i < max_i && j > 0 && is_symbol(arr[i+1][j-1])) ||
    (i < max_i && is_symbol(arr[i+1][j])) ||
    (i < max_i && j < max_j && is_symbol(arr[i+1][j+1]))
}

fn part1(contents: &String) {
    let mut arr: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        arr.push(row);
    }

    let mut result = 0;
    for (i, row) in arr.iter().enumerate() {
        let mut n = 0;
        let mut adj = false;
        for (j, c) in row.iter().enumerate() {
            if !c.is_digit(10) {
                if adj {
                    result += n;
                    adj = false;
                }
                n = 0;
                continue;
            }
            n = n * 10 + c.to_digit(10).expect("");
            adj = adj || surround_symbol(&arr, i, j);
        }
        if n != 0 && adj {result += n};
    }
    println!("{result}");
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

    for (i, row) in arr.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c != &'*' {continue;}
            let mut pos: Vec<(usize, usize)> = Vec::new();
            if j > 0 && arr[i][j-1].is_digit(10) {
                pos.push((i, j-1));
            }
            if j < max_j && arr[i][j+1].is_digit(10) {
                pos.push((i, j+1));
            }
            if i > 0 && arr[i-1][j].is_digit(10) {
                pos.push((i-1, j));
            } else {
                if i > 0 && j > 0 && arr[i-1][j-1].is_digit(10) {
                    pos.push((i-1, j-1));
                }
                if i > 0 && j < max_j && arr[i-1][j+1].is_digit(10) {
                    pos.push((i-1, j+1));
                }
            }
            if i < max_i && arr[i+1][j].is_digit(10) {
                pos.push((i+1, j));
            } else {
                if i < max_i && j > 0 && arr[i+1][j-1].is_digit(10) {
                    pos.push((i+1, j-1));
                }
                if i < max_i && j < max_j && arr[i+1][j+1].is_digit(10) {
                    pos.push((i+1, j+1));
                }
            }

            if pos.len() != 2 {continue;}
            let mut ratio = 1;
            for (i, j) in pos {
                let mut n = 0;
                let mut cur = j;
                while cur > 0 && arr[i][cur-1].is_digit(10) {
                    cur -= 1;
                }
                while cur <= max_j && arr[i][cur].is_digit(10) {
                    n = n * 10 + arr[i][cur].to_digit(10).expect("");
                    cur += 1;
                }
                ratio *= n;
            }
            result += ratio;
        }
    }
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day03.txt").expect("");

    part1(&contents);
    part2(&contents);
}