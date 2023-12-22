use std::fs;

fn printmap(map: &Vec<i32>) {
    for i in map {
        match i {
            1 => print!("#"),
            0 => print!("?"),
            -1 => print!("."),
            _ => panic!("Invalid num"),
        }
    }
}

fn consume(mut map: Vec<i32>, rec: &Vec<usize>, mut map_i: usize, mut rec_i: usize) -> u64 {
    // Attempt to set every index to '#' for current record length
    for i in 0..rec[rec_i] {
        // Interrupt if '.'
        if map_i+i >= map.len() || map[map_i+i] == -1 {
            // println!("ZERO: could not consume");
            return 0;
        }
        map[map_i+i] = 1;
    }
    // Interrupt if last index + 1 is a '#'
    map_i = map_i + rec[rec_i];
    if map_i < map.len() && map[map_i] == 1 {
        // println!("ZERO: exceeded consume");
        return 0;
    }

    // Set last index + 1 to '.'
    if map_i < map.len() {
        map[map_i] = -1;
    }

    // Increment current record
    rec_i += 1;
    // Check if record satisfied
    if rec_i == rec.len() {
        // Check there are no further '#' in map
        for i in 0..map.len()-map_i {
            if map[map_i+i] == 1 {
                // println!("ZERO: too many #");
                return 0;
            }
        }
        // printmap(&map);
        // println!(" Returned 1");
        return 1;
    }

    recursive(map, rec, map_i, rec_i)
}

fn consumable(map: &Vec<i32>, map_i: usize, cur_rec: usize) -> usize {
    for i in 0..cur_rec {
        if map_i+i == map.len() || map[map_i+i] == -1 {
            return 0;
        }
    }
    if map_i+cur_rec >= map.len() {
        map.len()
    } else if map[map_i+cur_rec] != 1 {
        map_i + cur_rec + 1
    }
    else {
        0
    }
}

fn update(dp: &mut Vec<Vec<(usize, u64)>>, dst: usize, rec_i: usize, perms: u64) {
    for i in 0..dp[dst].len() {
        if dp[dst][i].0 == rec_i {
            dp[dst][i].1 += perms;
            return;
        }
    }
    dp[dst].push((rec_i, perms));
}

fn solve(map: &Vec<i32>, rec: &Vec<usize>) -> u64 {
    let mut dp: Vec<Vec<(usize, u64)>> = Vec::new();
    for _ in 0..=map.len() {
        let dp_row: Vec<(usize, u64)> = Vec::new();
        dp.push(dp_row);
    }

    dp[0].push((0, 1));

    for map_i in 0..map.len() {
        let dp_cur = dp[map_i].clone();

        // If '.' or '?', advance by one
        if map[map_i] != 1 {
            for (rec_i, perms) in &dp_cur {
                update(&mut dp, map_i+1, *rec_i, *perms);
            }
        }

        // If '?' or '#', consume one
        if map[map_i] != -1 {
            for (rec_i, perms) in dp_cur {
                if rec_i == rec.len() {
                    continue;
                }
                let dst = consumable(map, map_i, rec[rec_i]);
                if dst == 0 {
                    continue;
                }
                update(&mut dp, dst, rec_i+1, perms);
            }
        }
    }
    
    for (rec_i, perms) in &dp[map.len()] {
        if rec_i == &rec.len() {
            return *perms
        }
    }
    panic!("No solution")
}

// Brute force checking
fn recursive(mut map: Vec<i32>, rec: &Vec<usize>, mut map_i: usize, rec_i: usize) -> u64 {
    // printmap(&map);
    // println!(" map_i: {map_i}, rec_i: {rec_i}, current rec: {}", rec[rec_i]);

    // while '.', search forward
    while map_i < map.len() && map[map_i] == -1 {
        map_i += 1;
    }

    if map_i >= map.len() {
        // println!("ZERO: map_i exceeded");
        return 0;
    }

    // if '#', find the total length of '#' section
    if map[map_i] == 1 {
        consume(map, rec, map_i, rec_i)
    }

    // If '?', split: add the next record and do not add the next record
    else {
        // println!("Splitting at index {map_i}");
        // Add the next record
        let map_cpy = map.clone();
        map[map_i] = -1;
        consume(map_cpy, rec, map_i, rec_i) + recursive(map, rec, map_i, rec_i)
    }
}

fn part1(contents: &String) {
    let mut result = 0;
    for line in contents.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let mut map: Vec<i32> = Vec::new();
        for c in split[0].chars() {
            map.push(match c {
                '#' => 1,
                '?' => 0,
                '.' => -1,
                _ => panic!("Invalid character")
            });
        }
        let mut rec: Vec<usize> = Vec::new();
        for num in split[1].split(",") {
            rec.push(num.parse::<usize>().expect("Should be integer"));
        }

        result += solve(&map, &rec);
    }

    println!("{result}");
}

fn part2(contents: &String) {
    let mut result = 0;

    for line in contents.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let mut map: Vec<i32> = Vec::new();
        let mut rec: Vec<usize> = Vec::new();
        for _ in 0..5 {
            for c in split[0].chars() {
                map.push(match c {
                    '#' => 1,
                    '?' => 0,
                    '.' => -1,
                    _ => panic!("Invalid character")
                });
            }
            map.push(0);
            for num in split[1].split(",") {
                rec.push(num.parse::<usize>().expect("Should be integer"));
            }
        }
        map.pop();
        
        result += solve(&map, &rec);
    }
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day12.txt").expect("");

    // part1(&contents);
    part1(&contents);
    part2(&contents);
}