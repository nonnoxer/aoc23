use std::{collections::{HashMap, VecDeque}, fs,};
use num::integer::lcm;

#[derive(Debug)]
enum State {
    Flip(bool),
    And(HashMap<usize, bool>),
    BCast,
    Misc,
}

fn solve(bcast: usize, outs: &Vec<Vec<usize>>, states: &mut Vec<State>) -> (i32, i32) {
    let mut queue: VecDeque<(usize, bool)> = VecDeque::new();
    let mut low = 0;
    let mut high = 0;

    queue.push_back((bcast, false));
    while queue.len() > 0 {
        let (i, pulse) = queue.pop_front().unwrap();
        if pulse {
            high += 1;
        } else {
            low += 1;
        }

        let mut out_pulse: bool = false;
        match &mut states[i] {
            State::Misc => {
                continue;
            }
            State::Flip(val) => {
                if pulse {
                    continue;
                }
                *val = !*val;
                out_pulse = *val;
            },
            State::And(sources) => {
                out_pulse = true;
                for (_, val) in sources {
                    out_pulse &= *val;
                }
                out_pulse = !out_pulse;
            },
            _ => (),
        }

        let out = &outs[i];
        for dst in out {
            match &mut states[*dst] {
                State::And(sources) => {
                    sources.insert(i, out_pulse);
                },
                _ => (),
            }
            queue.push_back((*dst, out_pulse));
        }
    }

    (low, high)
}

fn part1(contents: &String) {
    let mut map: HashMap<&str, usize> = HashMap::new();
    let mut raw_outs: Vec<Vec<&str>> = Vec::new();
    let mut outs: Vec<Vec<usize>> = Vec::new();
    let mut states: Vec<State> = Vec::new();
    let mut ands: Vec<usize> = Vec::new();
    let mut bcast = 0;
    
    for (i, line) in contents.lines().into_iter().enumerate() {
        let split: Vec<&str> = line.split(" -> ").collect();
        let dst: Vec<&str> = split[1].split(", ").collect();
        let state: State;
        let id: &str;
        if split[0] == "broadcaster" {
            state = State::BCast;
            id = split[0];
            bcast = i;
        } else {
            if &split[0][0..=0] == "%" {
                state = State::Flip(false);
            } else {
                state = State::And(HashMap::new());
                ands.push(i);
            }
            id = &split[0][1..];
        }
        map.insert(id, i);
        raw_outs.push(dst);
        states.push(state);
        outs.push(Vec::new());
    }
    let dummy = states.len();
    states.push(State::Misc);

    for i in 0..outs.len() {
        for dst in &raw_outs[i] {
            if !map.contains_key(dst) {
                outs[i].push(dummy);
                continue;
            }

            let cur = map[dst];
            outs[i].push(cur);

            if ands.contains(&cur) {
                match &mut states[cur] {
                    State::And(sources) => {
                        sources.insert(i, false);
                    },
                    _ => (),
                }
            }
        }
    }

    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let result = solve(bcast, &outs, &mut states);
        low += result.0;
        high += result.1;
    }

    println!("{}", low * high);
}

fn solve2(bcast: usize, outs: &Vec<Vec<usize>>, states: &mut Vec<State>, targets: &Vec<usize>, cycle: u64) {
    let mut queue: VecDeque<(usize, bool)> = VecDeque::new();

    queue.push_back((bcast, false));
    while queue.len() > 0 {
        let (i, pulse) = queue.pop_front().unwrap();
        let mut out_pulse: bool = false;

        if targets.contains(&i) && !pulse {//vec![3917, 3931, 3943, 4057].contains(&cycle) {
            println!("{cycle}: {i} {pulse}");
        }

        match &mut states[i] {
            State::Misc => {
                continue;
            }
            State::Flip(val) => {
                if pulse {
                    continue;
                }
                *val = !*val;
                out_pulse = *val;
            },
            State::And(sources) => {
                out_pulse = true;
                for (_, val) in sources {
                    out_pulse &= *val;
                }
                out_pulse = !out_pulse;
            },
            _ => (),
        }

        let out = &outs[i];
        for dst in out {
            match &mut states[*dst] {
                State::And(sources) => {
                    sources.insert(i, out_pulse);
                },
                _ => (),
            }
            queue.push_back((*dst, out_pulse));
        }
    }
}

fn part2(contents: &String) {
    let mut map: HashMap<&str, usize> = HashMap::new();
    let mut raw_outs: Vec<Vec<&str>> = Vec::new();
    let mut outs: Vec<Vec<usize>> = Vec::new();
    let mut states: Vec<State> = Vec::new();
    let mut ands: Vec<usize> = Vec::new();
    let mut bcast = 0;

    let raw_targets = vec!["vt", "sk", "xc", "kk"];
    let mut targets: Vec<usize> = Vec::new();
    
    for (i, line) in contents.lines().into_iter().enumerate() {
        let split: Vec<&str> = line.split(" -> ").collect();
        let dst: Vec<&str> = split[1].split(", ").collect();
        let state: State;
        let id: &str;
        if split[0] == "broadcaster" {
            state = State::BCast;
            id = split[0];
            bcast = i;
        } else {
            if &split[0][0..=0] == "%" {
                state = State::Flip(false);
            } else {
                state = State::And(HashMap::new());
                ands.push(i);
            }
            id = &split[0][1..];
            if raw_targets.contains(&id) {
                targets.push(i);
            }
        }
        map.insert(id, i);
        raw_outs.push(dst);
        states.push(state);
        outs.push(Vec::new());
    }
    let dummy = states.len();
    states.push(State::Misc);

    for i in 0..outs.len() {
        for dst in &raw_outs[i] {
            if !map.contains_key(dst) {
                outs[i].push(dummy);
                continue;
            }

            let cur = map[dst];
            outs[i].push(cur);

            if ands.contains(&cur) {
                match &mut states[cur] {
                    State::And(sources) => {
                        sources.insert(i, false);
                    },
                    _ => (),
                }
            }
        }
    }

    // print!("Targets: ");
    // for target in &targets {
    //     print!("{target} ");
    // }
    // println!();
    // let mut cycle = 0;
    let mut result: u64 = 1;
    // for _ in 0..10000 {
    //     cycle += 1;
    //     solve2(bcast, &outs, &mut states, &targets, cycle);
    // }
    for num in [3917, 3931, 3943, 4057] {
        result = lcm(result, num);
    }
    println!("{result}");
}

pub fn main() {
    let contents = fs::read_to_string("input/day20.txt").expect("");

    part1(&contents);
    part2(&contents);
}
