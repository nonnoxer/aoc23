use std::fs;

fn part1(contents: &String) {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(match c {
                'O' => 2,
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid character"),
            });
        }
        arr.push(row);
    }

    let mut result = 0;
    for j in 0..arr[0].len() {
        let mut cur = arr.len();
        for i in 0..arr.len() {
            if arr[i][j] == 1 {
                cur = arr.len() - i - 1;
            }
            if arr[i][j] == 2 {
                result += cur;
                cur -= 1;
            }
        }
    }
    println!("{result}");
}

fn cycle(arr: &mut Vec<Vec<i32>>) {
    for j in 0..arr[0].len() {
        let mut cur = 0;
        for i in 0..arr.len() {
            if arr[i][j] == 1 {
                cur = i + 1;
            }
            if arr[i][j] == 2 {
                arr[i][j] = 0;
                arr[cur][j] = 2;
                cur += 1;
            }
        }
    }
    for i in 0..arr.len() {
        let mut cur = 0;
        for j in 0..arr[0].len() {
            if arr[i][j] == 1 {
                cur = j + 1;
            }
            if arr[i][j] == 2 {
                arr[i][j] = 0;
                arr[i][cur] = 2;
                cur += 1;
            }
        }
    }
    for j in 0..arr[0].len() {
        let mut cur = arr.len() - 1;
        for i in (0..arr.len()).rev() {
            if i > 0 && arr[i][j] == 1 {
                cur = i - 1;
            }
            if arr[i][j] == 2 {
                arr[i][j] = 0;
                arr[cur][j] = 2;
                if cur > 0 {
                    cur -= 1;
                }
            }
        }
    }
    for i in 0..arr.len() {
        let mut cur = arr[0].len() - 1;
        for j in (0..arr[0].len()).rev() {
            if j > 0 && arr[i][j] == 1 {
                cur = j - 1;
            }
            if arr[i][j] == 2 {
                arr[i][j] = 0;
                arr[i][cur] = 2;
                if cur > 0 {
                    cur -= 1;
                }
            }
        }
    }
}

fn part2(contents: &String) {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(match c {
                'O' => 2,
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid character"),
            });
        }
        arr.push(row);
    }

    for _ in 0..1000 {
        let mut arr_cpy = arr.clone();
        cycle(&mut arr_cpy);
        // let mut diff = 0;
        // for i in 0..arr.len() {
        //     for j in 0..arr[0].len() {
        //         if arr[i][j] != arr_cpy[i][j] {
        //             diff += 1;
        //         }
        //     }
        // }
        // println!("{} {diff}", i+1);
        arr = arr_cpy;
    }

    let mut result = 0;
    for j in 0..arr[0].len() {
        for i in 0..arr.len() {
            if arr[i][j] == 2 {
                result += arr.len() - i;
            }
        }
    }
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day14.txt").expect("");

    part1(&contents);
    part2(&contents);
}