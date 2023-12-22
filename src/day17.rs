use std::{fs, collections::VecDeque};


fn do_part1(arr: &Vec<Vec<i32>>, min: usize, max: usize) -> i32 {
    let max_i = arr.len()-1;
    let max_j = arr[0].len()-1;

    let mut dp: Vec<Vec<(i32, i32)>> = Vec::new();
    for _ in 0..=max_i {
        let mut dp_row: Vec<(i32, i32)> = Vec::new();
        for _ in 0..=max_j {
            dp_row.push((0, 0));
        }
        dp.push(dp_row);
    }

    let mut queue: VecDeque<(usize, usize, i32, i32)> = VecDeque::new();
    queue.push_back((0, 0, 0, 0));
    queue.push_back((0, 0, 0, 1));
    while queue.len() > 0 {
        let (i, j, cost, dir) = queue.pop_front().unwrap();
        match dir {
            0 => {
                if dp[i][j].0 != 0 && cost >= dp[i][j].0 {
                    continue;
                }
                dp[i][j].0 = cost;
            },
            1 => {
                if dp[i][j].1 != 0 && cost >= dp[i][j].1 {
                    continue;
                }
                dp[i][j].1 = cost;
            },
            _ => (),
        }
        
        match dir {
            0 => { // horizontal
                let mut temp = cost;
                for k in 1..min {
                    if j+k > max_j {
                        break;
                    }
                    temp += arr[i][j+k];
                }
                for k in min..=max {
                    if j+k > max_j {
                        break;
                    }
                    temp += arr[i][j+k];
                    if dp[i][j+k].1 == 0 || temp < dp[i][j+k].1 {
                        queue.push_back((i, j+k, temp, 1));
                    }
                }
                temp = cost;
                for k in 1..min {
                    if j < k {
                        break;
                    }
                    temp += arr[i][j-k];
                }
                for k in min..=max {
                    if j < k {
                        break;
                    }
                    temp += arr[i][j-k];
                    if dp[i][j-k].1 == 0 || temp < dp[i][j-k].1 {
                        queue.push_back((i, j-k, temp, 1));
                    }
                }
            },
            1 => { // vertical
                let mut temp = cost;
                for k in 1..min {
                    if i+k > max_i {
                        break;
                    }
                    temp += arr[i+k][j];
                }
                for k in min..=max {
                    if i+k > max_i {
                        break;
                    }
                    temp += arr[i+k][j];
                    if dp[i+k][j].0 == 0 || temp < dp[i+k][j].0 {
                        queue.push_back((i+k, j, temp, 0));
                    }
                }
                temp = cost;
                for k in 1..min {
                    if i < k {
                        break;
                    }
                    temp += arr[i-k][j];
                }
                for k in min..=max {
                    if i < k {
                        break;
                    }
                    temp += arr[i-k][j];
                    if dp[i-k][j].0 == 0 || temp < dp[i-k][j].0 {
                        queue.push_back((i-k, j, temp, 0));
                    }
                }
            },
            _ => (),
        }
    }

    let max = dp[max_i][max_j];
    if max.0 > max.1 {
        max.1
    } else {
        max.0
    }
}

fn part1(contents: &String) {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).expect("Should be number") as i32);
        }
        arr.push(row);
    }

    let result = do_part1(&arr, 1, 3);
    println!("{result}");
}

fn part2(contents: &String) {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).expect("Should be number") as i32);
        }
        arr.push(row);
    }

    let result = do_part1(&arr, 4, 10);
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day17.txt").expect("");

    part1(&contents);
    part2(&contents);
}