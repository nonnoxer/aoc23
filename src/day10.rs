use std::{fs, collections::VecDeque};

fn part1(contents: &String) -> Vec<(usize, usize)> {
    let mut arr: Vec<Vec<char>> = Vec::new();
    let mut prev: (usize, usize) = (0, 0);
    let mut cur: (usize, usize) = (0, 0);
    let mut next: (usize, usize);
    let mut pipe: Vec<(usize, usize)> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                prev = (i, j);
                cur = (i, j - 1);
                pipe.push((i, j));
            }
        }
        arr.push(row);
    }
    let mut steps = 1;
    while arr[cur.0][cur.1] != 'S' {
        pipe.push(cur);
        steps += 1;
        next = match arr[cur.0][cur.1] {
            '|' => {
                if cur.0 > prev.0 {(cur.0+1, cur.1)}
                else {(cur.0-1, cur.1)}
            },
            '-' => {
                if cur.1 > prev.1 {(cur.0, cur.1+1)}
                else {(cur.0, cur.1-1)}
            },
            'L' => {
                if cur.0 > prev.0 {(cur.0, cur.1+1)}
                else {(cur.0-1, cur.1)}
            },
            'J' => {
                if cur.0 > prev.0 {(cur.0, cur.1-1)}
                else {(cur.0-1, cur.1)}
            },
            '7' => {
                if cur.0 < prev.0 {(cur.0, cur.1-1)}
                else {(cur.0+1, cur.1)}
            },
            'F' => {
                if cur.0 < prev.0 {(cur.0, cur.1+1)}
                else {(cur.0+1, cur.1)}
            },
            'S' => break,
            _ => panic!("Invalid character"),
        };
        prev = cur;
        cur = next;
    }
    let result = (steps + 1) / 2;
    println!("{result}");
    pipe
}

fn add(cur: (usize, usize), enc: &mut Vec<Vec<i32>>, buf: &mut VecDeque<(usize, usize)>) {
    if cur.0 == 0 || cur.1 == 0 || cur.0 >= enc.len() || cur.1 >= enc[0].len() {
        panic!("Edge detected ({}, {})", cur.0, cur.1);
    }
    // println!("Adding ({}, {})", cur.0, cur.1);
    if enc[cur.0][cur.1] == 0 {
        enc[cur.0][cur.1] = 1;
        buf.push_back(cur);
    }
}

fn add_area(coord: (usize, usize), enc: &mut Vec<Vec<i32>>, buf: &mut VecDeque<(usize, usize)>, result: &mut i32) {
    if coord.0 == 0 || coord.1 == 0 || coord.0 >= enc.len() || coord.1 >= enc[0].len() {
        panic!("Edge detected ({}, {})", coord.0, coord.1);
    }
    if enc[coord.0][coord.1] == 0 || enc[coord.0][coord.1+1] == 0 || enc[coord.0+1][coord.1] == 0 || enc[coord.0+1][coord.1+1] == 0 {
        *result += 1;
        add((coord.0, coord.1), enc, buf);
        add((coord.0, coord.1+1), enc, buf);
        add((coord.0+1, coord.1), enc, buf);
        add((coord.0+1, coord.1+1), enc, buf);
    }
}

fn part2(contents: &String) {
    let mut arr: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    for (i, line) in contents.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
                row.push('-');
            }
            else {
                row.push(c);
            }
        }
        arr.push(row);
    }

    let pipe = part1(contents);

    // encoded coordinate: BFS for "between" coordinates
    // (x, y)        (x, y+1)
    //       enc(x,y)
    // (x+1,y)       (x+1, y+1)
    let mut enc: Vec<Vec<i32>> = Vec::new();
    for _ in 0..arr.len() {
        let mut enc_row: Vec<i32> = Vec::new();
        for _ in 0..arr[0].len() {
            enc_row.push(0);
        }
        enc.push(enc_row);
    }

    enc[start.0+1][start.1] = 1;
    let mut buf: VecDeque<(usize, usize)> = VecDeque::new();
    buf.push_back((start.0+1, start.1));
    let mut result = 0;

    while buf.len() > 0 {
        let cur = buf.pop_front().expect("Should exist");
        // println!("{} {} ({}, {})", arr[cur.0-1][cur.1-1], arr[cur.0-1][cur.1], cur.0, cur.1);
        // println!("{} {}", arr[cur.0][cur.1-1], arr[cur.0][cur.1]);
        if !pipe.contains(&(cur.0-1, cur.1-1)) || arr[cur.0-1][cur.1-1] == '.' {
            add_area((cur.0-1, cur.1-1), &mut enc, &mut buf, &mut result)
        } else {match arr[cur.0-1][cur.1-1] {
            '|' | '7' => add((cur.0-1, cur.1), &mut enc, &mut buf),
            '-' | 'L' => add((cur.0, cur.1-1), &mut enc, &mut buf),
            'J' => {
                add((cur.0-1, cur.1), &mut enc, &mut buf);
                add((cur.0, cur.1-1), &mut enc, &mut buf);
            },
            _ => (),
        }}
        if !pipe.contains(&(cur.0-1, cur.1)) || arr[cur.0-1][cur.1] == '.' {
            add_area((cur.0-1, cur.1), &mut enc, &mut buf, &mut result)
        } else {match arr[cur.0-1][cur.1] {
            '|' | 'F' => add((cur.0-1, cur.1), &mut enc, &mut buf),
            '-' | 'J' => add((cur.0, cur.1+1), &mut enc, &mut buf),
            'L' => {
                add((cur.0-1, cur.1), &mut enc, &mut buf);
                add((cur.0, cur.1+1), &mut enc, &mut buf);
            },
            _ => (),
        }}
        if !pipe.contains(&(cur.0, cur.1-1)) || arr[cur.0][cur.1-1] == '.' {
            add_area((cur.0, cur.1-1), &mut enc, &mut buf, &mut result)
        } else {match arr[cur.0][cur.1-1] {
            '|' | 'J' => add((cur.0+1, cur.1), &mut enc, &mut buf),
            '-' | 'F' => add((cur.0, cur.1-1), &mut enc, &mut buf),
            '7' => {
                add((cur.0+1, cur.1), &mut enc, &mut buf);
                add((cur.0, cur.1-1), &mut enc, &mut buf);
            },
            _ => (),
        }}
        if !pipe.contains(&(cur.0, cur.1)) || arr[cur.0][cur.1] == '.' {
            add_area((cur.0, cur.1), &mut enc, &mut buf, &mut result)
        } else {match arr[cur.0][cur.1] {
            '|' | 'L' => add((cur.0+1, cur.1), &mut enc, &mut buf),
            '-' | '7' => add((cur.0, cur.1+1), &mut enc, &mut buf),
            'F' => {
                add((cur.0+1, cur.1), &mut enc, &mut buf);
                add((cur.0, cur.1+1), &mut enc, &mut buf);
            },
            _ => (),
        }
    }}
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day10.txt").expect("Should have been able to read file");

    // part1(&content );
    part2(&contents);
}