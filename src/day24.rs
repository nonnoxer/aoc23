use std::fs;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn mul_con(p: &Point, c: f64) -> Point {
    Point{
        x: p.x * c,
        y: p.y * c,
        z: p.z * c,
    }
}

fn sub(p1: &Point, p2: &Point) -> Point {
    Point{
        x: p1.x - p2.x,
        y: p1.y - p2.y,
        z: p1.z - p2.z,
    }
}

fn add(p1: &Point, p2: &Point) -> Point {
    Point{
        x: p1.x + p2.x,
        y: p1.y + p2.y,
        z: p1.z + p2.z,
    }
}

fn dot(p1: &Point, p2: &Point) -> f64 {
    p1.x*p2.x + p1.y*p2.y + p1.z*p2.z
}

fn cross(p1: &Point, p2: &Point) -> Point {
    Point{
        x: p1.y*p2.z - p1.z*p2.y,
        y: p1.z*p2.x - p1.x*p2.z,
        z: p1.x*p2.y - p1.y*p2.x,
    }
}

fn parallel(v1: &Point, v2: &Point) -> bool {
    v1.y*v2.z == v1.z*v2.y && v1.z*v2.x == v1.x*v2.z && v1.x*v2.y == v1.y*v2.x
}

fn solve(p1: &Point, v1: &Point, p2: &Point, v2: &Point, min: f64, max: f64) -> i32 {
    if parallel(&v1, &v2) {
        // println!("parallel");
        if !parallel(&sub(&p1, &p2), &v1) {
            return 0;
        }
        // println!("collinear");
        if (((p1.x < min && v1.x < 0.) || (p1.x > max && v1.x > 0.)) ||
            ((p2.x < min && v2.x < 0.) || (p2.x > max && v2.x > 0.))) ||
           (((p1.y < min && v1.y < 0.) || (p1.y > max && v1.y > 0.)) ||
            ((p2.y < min && v2.y < 0.) || (p2.y > max && v2.y > 0.))) {
                return 0;
        }
        return 1;
    }
    let l = (p2.y - p1.y + (p1.x - p2.x) * v2.y / v2.x) / (v1.y - v1.x * v2.y / v2.x);
    let m = (l * v1.x + p1.x - p2.x) / v2.x;
    if l < 0.0 || m < 0.0 {
        return 0;
    }
    let pi = Point{
        x: p1.x + l*v1.x,
        y: p1.y + l*v1.y,
        z: p1.z + l*v1.z,
    }; 
    if pi.x < min || pi.x > max || pi.y < min || pi.y > max {
        return 0;
    }
    1
}

fn part1(contents: &String) {
    let mut positions: Vec<Point> = Vec::new();
    let mut velocity: Vec<Point> = Vec::new();

    let mut result = 0;
    for line in contents.lines() {
        let split: Vec<&str> = line.split(" @ ").collect();
        let pos_str: Vec<&str> = split[0].split(", ").collect();
        let vel_str: Vec<&str> = split[1].split(", ").collect();
        let mut pos_arr = [0.0; 3];
        let mut vel_arr = [0.0; 3];
        for i in 0..2 {
            pos_arr[i] = pos_str[i].trim().parse::<f64>().expect("Should be number");
            vel_arr[i] = vel_str[i].trim().parse::<f64>().expect("Should be number");
        }
        let pos = Point{x: pos_arr[0], y: pos_arr[1], z: pos_arr[2]};
        let vel = Point{x: vel_arr[0], y: vel_arr[1], z: vel_arr[2]};

        for i in 0..positions.len() {
            result += solve(&pos, &vel, &positions[i], &velocity[i], 200000000000000.0, 400000000000000.0);
        }

        positions.push(pos);
        velocity.push(vel);
    }
    println!("{result}");
}

fn part2(contents: &String) {
    let mut positions: Vec<Point> = Vec::new();
    let mut velocity: Vec<Point> = Vec::new();

    for line in contents.lines() {
        let split: Vec<&str> = line.split(" @ ").collect();
        let pos_str: Vec<&str> = split[0].split(", ").collect();
        let vel_str: Vec<&str> = split[1].split(", ").collect();
        let mut pos_arr = [0.0; 3];
        let mut vel_arr = [0.0; 3];
        for i in 0..3 {
            pos_arr[i] = pos_str[i].trim().parse::<f64>().expect("Should be number");
            vel_arr[i] = vel_str[i].trim().parse::<f64>().expect("Should be number");
        }
        let pos = Point{x: pos_arr[0], y: pos_arr[1], z: pos_arr[2]};
        let vel = Point{x: vel_arr[0], y: vel_arr[1], z: vel_arr[2]};
    
        positions.push(pos);
        velocity.push(vel);
    }

    let p1 = &positions[0];
    let v1 = &velocity[0];

    // Point 2 relative to point 1
    let p2 = &sub(&positions[1], p1);
    let v2 = &sub(&velocity[1], v1);

    // Plane passing through origin and p2
    let n = &cross(p2, v2);

    // Point 3 and 4 relative to point 1
    let p3 = &sub(&positions[2], p1);
    let v3 = &sub(&velocity[2], v1);
    let p4 = &sub(&positions[3], p1);
    let v4 = &sub(&velocity[3], v1);

    // Intersection of p3 and p4 with plane
    let l = -dot(n, p3) / dot(n, v3);
    let m = -dot(n, p4) / dot(n, v4);
    let i3 = &add(p3, &mul_con(v3, l));
    let i4 = &add(p4, &mul_con(v4, m));

    // Calculate rock trajectory
    if l == m {
        panic!("equal");
    } 
    let v = &add(&mul_con(&sub(i4, i3),1. / (m - l)), v1);
    let p = &add(&add(&positions[2], &mul_con(&velocity[2], l)), &mul_con(v, -l));

    let result = p.x + p.y + p.z;
    println!("{result}")
}

pub fn main() {
    let contents = fs::read_to_string("input/day24.txt").expect("");

    part1(&contents);
    part2(&contents);
}