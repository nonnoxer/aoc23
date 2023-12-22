use std::{fs, collections::HashSet};


fn part1(contents: &String) {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    let mut start = (0, 0);
    for (i, line) in contents.lines().into_iter().enumerate() {
        let mut row: Vec<i32> = Vec::new();
        for (j, c) in line.chars().into_iter().enumerate() {
            row.push(match c {
                '.' => 0,
                '#' => 1,
                'S' => {
                    start = (i, j);
                    0
                },
                _ => panic!("Invalid character"),
            });
        }
        arr.push(row);
    }

    let mut one_back: HashSet<(usize, usize)> = HashSet::new();
    let mut cur: HashSet<(usize, usize)> = HashSet::new();
    cur.insert(start);

    for _ in 0..64 {
        (_, one_back, cur) = step(&arr, one_back, cur);
    }

    let result = cur.len();
    println!("{result}")
}

fn step(
    arr: &Vec<Vec<i32>>,
    one_back: HashSet<(usize, usize)>,
    cur: HashSet<(usize, usize)>
) -> (usize, HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut next = one_back;
    for (i, j) in &cur {
        let i = *i;
        let j = *j;
        if i > 0 && arr[i-1][j] == 0 && !next.contains(&(i-1, j)) {
            next.insert((i-1, j));
        }
        if i < arr.len()-1 && arr[i+1][j] == 0 && !next.contains(&(i+1, j)) {
            next.insert((i+1, j));
        }
        if j > 0 && arr[i][j-1] == 0 && !next.contains(&(i, j-1)) {
            next.insert((i, j-1));
        }
        if j < arr[0].len()-1 && arr[i][j+1] == 0 && !next.contains(&(i, j+1)) {
            next.insert((i, j+1));
        }
    }

    (cur.len(), cur, next)
}

fn insert_overflow(arr: &Vec<Vec<i32>>, next: &mut HashSet<(i32, i32)>, x: i32, y: i32) {
    let i = (((x % 131) + 131) % 131) as usize;
    let j = (((y % 131) + 131) % 131) as usize;
    if arr[i][j] == 0 && !next.contains(&(x, y)) {
        next.insert((x, y));
    }
}

fn step_overflow(
    arr: &Vec<Vec<i32>>,
    one_back: HashSet<(i32, i32)>,
    cur: HashSet<(i32, i32)>
) -> (usize, HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
    let mut next = one_back;
    for (x, y) in &cur {
        let x = *x;
        let y = *y;
        insert_overflow(arr, &mut next, x-1, y);
        insert_overflow(arr, &mut next, x+1, y);
        insert_overflow(arr, &mut next, x, y-1);
        insert_overflow(arr, &mut next, x, y+1);
    }

    (cur.len(), cur, next)
}

fn steps_to_fill(arr: &Vec<Vec<i32>>, start: (usize, usize)) -> usize {
    let mut one_back: HashSet<(usize, usize)> = HashSet::new();
    let mut cur: HashSet<(usize, usize)> = HashSet::new();
    cur.insert(start);

    let mut result = 0;
    let mut prev = 0;
    let mut prev_two = 0;
    let mut current = 0;

    loop {
        result += 1;
        prev_two = prev;
        prev = current;
        (current, one_back, cur) = step(&arr, one_back, cur);
        if current == prev_two && result % 2 == 0 {
            return result - 2;
        }
    }
}

fn run_steps(arr: &Vec<Vec<i32>>, start: (usize, usize), steps: usize) -> usize {
    let mut one_back: HashSet<(usize, usize)> = HashSet::new();
    let mut cur: HashSet<(usize, usize)> = HashSet::new();
    cur.insert(start);

    for _ in 0..steps {
        (_, one_back, cur) = step(&arr, one_back, cur);
    }

    cur.len()
}

fn run_steps_overflow(arr: &Vec<Vec<i32>>, start: (i32, i32), steps: usize) -> usize {
    let mut one_back: HashSet<(i32, i32)> = HashSet::new();
    let mut cur: HashSet<(i32, i32)> = HashSet::new();
    cur.insert(start);

    for _ in 0..steps {
        (_, one_back, cur) = step_overflow(&arr, one_back, cur);
    }

    cur.len()
}

fn part2(contents: &String) {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    let mut start = (0, 0);
    for (i, line) in contents.lines().into_iter().enumerate() {
        let mut row: Vec<i32> = Vec::new();
        for (j, c) in line.chars().into_iter().enumerate() {
            row.push(match c {
                '.' => 0,
                '#' => 1,
                'S' => {
                    start = (i, j);
                    0
                },
                _ => panic!("Invalid character"),
            });
        }
        arr.push(row);
    }

    // println!("{}", run_steps(&arr, (65, 65), 131));
    // let cycle_steps = 131; // = steps_to_fill(&arr, start);
    let full_e = 7523; // run_steps(&arr, start, 129);
    let full_o = 7584; // run_steps(&arr, start, 130);
    let t = 5690;
    let r = 5684;
    let b = 5680;
    let l = 5686;
    let tr_corner = 940;
    let br_corner = 966;
    let bl_corner = 968;
    let tl_corner = 967;
    let tr_wedge = 6612;
    let br_wedge = 6595;
    let bl_wedge = 6608;
    let tl_wedge = 6601;
    let n = 202300; // 26501365 / 131

    let result: u64 = 
        n*n*full_o +
        (n-1)*(n-1)*full_e +
        n*(tr_corner + br_corner + bl_corner + tl_corner) +
        (n-1)*(tr_wedge + br_wedge + bl_wedge + tl_wedge) +
        t + r + b + l;

    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day21.txt").expect("");

    part1(&contents);
    part2(&contents);
}