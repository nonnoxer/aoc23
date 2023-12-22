use std::fs;

struct Block {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
    safe: bool,
    bot: Vec<usize>,
    top: Vec<usize>,
}

fn setup(contents: &String) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    for line in contents.lines() {
        let coords: Vec<&str> = line.split([',', '~']).collect();
        let mut coords_num: Vec<usize> = Vec::new();
        for coord in coords {
            coords_num.push(coord.parse().expect("Should be number"));
        }
        blocks.push(Block{
            x: (coords_num[0], coords_num[3]),
            y: (coords_num[1], coords_num[4]),
            z: (coords_num[2], coords_num[5]),
            safe: true,
            bot: Vec::new(),
            top: Vec::new(),
        });
    }
    blocks.sort_by(|a, b| a.z.0.cmp(&b.z.0));

    let mut dp: [[(usize, i32); 10]; 10] = [[(0, -1); 10]; 10];
    
    for k in 0..blocks.len() {
        let block = &mut blocks[k];
        let mut min_z = 0;
        let mut min_z_blocks: Vec<usize> = Vec::new();
        for i in block.x.0..=block.x.1 {
            for j in block.y.0..=block.y.1 {
                if dp[i][j].0 > min_z {
                    min_z = dp[i][j].0;
                    min_z_blocks.clear();
                    min_z_blocks.push(dp[i][j].1 as usize);
                } else if dp[i][j].0 > 0 && dp[i][j].0 == min_z && !min_z_blocks.contains(&(dp[i][j].1 as usize)) {
                    min_z_blocks.push(dp[i][j].1 as usize);
                }
            }
        }
        let z = min_z + block.z.1 - block.z.0 + 1;
        for i in block.x.0..=block.x.1 {
            for j in block.y.0..=block.y.1 {
                dp[i][j].0 = z;
                dp[i][j].1 = k as i32;
            }
        }
        if min_z_blocks.len() == 1 {
            blocks[min_z_blocks[0] as usize].safe = false;
        }
        for i in &min_z_blocks {
            blocks[*i].top.push(k);
        }
        blocks[k].bot = min_z_blocks;
    }
    blocks
}

fn part1(contents: &String) {
    let blocks = setup(contents);
    
    let mut result = 0;
    for block in &blocks {
        if block.safe {
            result += 1;
        }
    }
    println!("{result}");
}

fn cascade(blocks: &Vec<Block>, dependencies: &mut Vec<usize>, i: usize) -> u64 {
    if blocks[i].top.len() == 0 {
        return 0;
    }
    let mut result = 0;
    for k in &blocks[i].top {
        dependencies[*k] -= 1;
        if dependencies[*k] == 0 {
            result += cascade(blocks, dependencies, *k) + 1;
        }
    }
    result
}

fn part2(contents: &String) {
    let blocks = setup(contents);
    let mut result = 0;
    let mut dependencies: Vec<usize> = Vec::new();
    for i in 0..blocks.len() {
        dependencies.push(blocks[i].bot.len());
    }

    for i in 0..blocks.len() {
        if blocks[i].safe {
            continue;
        }
        let mut dep_cpy = dependencies.clone();
        result += cascade(&blocks, &mut dep_cpy, i);
    }

    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day22.txt").expect("");

    part1(&contents);
    part2(&contents);
}