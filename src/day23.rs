use std::{fs, collections::{VecDeque, HashMap}};


fn insert(dp: &mut Vec<Vec<i32>>, queue: &mut VecDeque<(usize, usize, i32)>, i: usize, j: usize, dir: i32, val: i32) {
    if val > dp[i][j] {
        dp[i][j] = val;
        queue.push_back((i, j, dir));
    }
}

fn part1(contents: &String) {
    let mut arr: Vec<Vec<char>> = Vec::new();
    let mut dp: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        let mut dp_row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c);
            dp_row.push(0);
        }
        arr.push(row);
        dp.push(dp_row);
    }

    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    queue.push_back((0, 1, 1));
    while queue.len() > 0 {
        let (i, j, dir) = queue.pop_front().unwrap();
        let cur_steps = dp[i][j];
        let cur_char = arr[i][j];
        if cur_char == '.' || cur_char == '>' {
            if dir != 2 && arr[i][j+1] != '#' && arr[i][j+1] != '<' {
                insert(&mut dp, &mut queue, i, j+1, 0, cur_steps+1);
            }
        }
        if cur_char == '.' || cur_char == 'v' {
            if dir != 3 && i < arr.len()-1 && arr[i+1][j] != '#' && arr[i+1][j] != '<' {
                insert(&mut dp, &mut queue, i+1, j, 1, cur_steps+1);
            }
        }
        if cur_char == '.' || cur_char == '<' {
            if dir != 0 && arr[i][j-1] != '#' && arr[i][j-1] != '>' {
                insert(&mut dp, &mut queue, i, j-1, 2, cur_steps+1);
            }
        }
        if cur_char == '.' || cur_char == '^' {
            if dir != 1 && arr[i-1][j] != '#' && arr[i-1][j] != '<' {
                insert(&mut dp, &mut queue, i-1, j, 3, cur_steps+1);
            }
        }
    }
    let result = dp[dp.len()-1][dp[0].len()-2];
    println!("{result}");
}

fn is_inter(arr: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    arr[i][j-1] + arr[i][j+1] + if i > 0 {arr[i-1][j]} else {-4} + if i+1 < arr.len() {arr[i+1][j]} else {-4} < 2
}

fn dfs(arr: &Vec<Vec<i32>>, i: usize, j: usize, dir: i32) -> (usize, usize, i32) {
    if is_inter(arr, i, j) {
        (i, j, 1)
    } else if dir != 2 && arr[i][j+1] == 0 {
        let (x, y, val) = dfs(arr, i, j+1, 0);
        (x, y, val + 1)
    } else if dir != 3 && arr[i+1][j] == 0 {
        let (x, y, val) = dfs(arr, i+1, j, 1);
        (x, y, val + 1)
    } else if dir != 0 && arr[i][j-1] == 0 {
        let (x, y, val) = dfs(arr, i, j-1, 2);
        (x, y, val + 1)
    } else if dir != 1 && arr[i-1][j] == 0 {
        let (x, y, val) = dfs(arr, i-1, j, 3);
        (x, y, val + 1)
    } else {
        panic!("Dead end")
    }
}
    
fn part2(contents: &String) {
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '#' => 1,
                _ => 0,
            });
        }
        arr.push(row);
    }

    // Find and encode intersections
    let mut inters: HashMap<(usize, usize), usize> = HashMap::new();
    let mut adj_list: Vec<Vec<(usize, i32)>> = Vec::new();
    let mut dp: Vec<i32> = Vec::new();
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if arr[i][j] == 0 && is_inter(&arr, i, j) {
                inters.insert((i, j), adj_list.len());
                adj_list.push(Vec::new());
                dp.push(-1);
            }
        }
    }
    let end = *inters.get(&(arr.len()-1, arr[0].len()-2)).unwrap();
    // println!("{end}");
    // println!("{inters:#?}");

    // Create adjacency list
    for ((i, j), k) in &inters {
        let i = *i;
        let j = *j;
        let k = *k;
        if arr[i][j+1] == 0 {
            let (x, y, val) = dfs(&arr, i, j+1, 0);
            adj_list[k].push((*inters.get(&(x, y)).unwrap(), val));
        }
        if i+1 < arr[0].len() && arr[i+1][j] == 0 {
            let (x, y, val) = dfs(&arr, i+1, j, 1);
            adj_list[k].push((*inters.get(&(x, y)).unwrap(), val));
        }
        if arr[i][j-1] == 0 {
            let (x, y, val) = dfs(&arr, i, j-1, 2);
            adj_list[k].push((*inters.get(&(x, y)).unwrap(), val));
        }
        if i > 0 && arr[i-1][j] == 0 {
            let (x, y, val) = dfs(&arr, i-1, j, 3);
            adj_list[k].push((*inters.get(&(x, y)).unwrap(), val));
        }
    }

    // for adj in &mut adj_list {
    //     adj.sort_by(|a, b| a.1.cmp(&b.1));
    //     println!("{adj:#?}");
    // }

    let mut queue: VecDeque<(usize, i32, Vec<usize>)> = VecDeque::new();
    queue.push_back((0, 0, vec![0]));

    let mut result = 0;
    while queue.len() > 0 {
        let (cur, val, history) = queue.pop_front().unwrap();
        if cur == end && val > result {
            result = val;
        }
        dp[cur] = val;
        // print!("{cur}, {val}: ");
        for (k, steps) in &adj_list[cur] {
            if !history.contains(k) && val + steps >= dp[*k]-100 {
                // print!("{k} ");
                let mut history_cpy = history.clone();
                history_cpy.push(*k);
                queue.push_back((*k, val+steps, history_cpy));
            }
        }
        // println!();
    }

    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day23.txt").expect("");

    part1(&contents);
    part2(&contents);
}