fn quadratic(b: f64, c: f64) -> i64 {
    // t(TIME - t) > DISTANCE
    // ie t^2 - TIME*t + DISTANCE < 0
    let lower = ((b - (b*b - 4.0*c).sqrt()) * 0.5).ceil() as i64;
    let upper = ((b + (b*b - 4.0*c).sqrt()) * 0.5).floor() as i64;
    upper - lower + 1
}

fn part1() {
    let time = vec![50, 74, 86, 85];
    let dist = vec![242, 1017, 1691, 1252];
    let mut result = 1;
    for i in 0..4 {
        let b = time[i] as f64;
        let c = dist[i] as f64;
        result *= quadratic(b, c);
    }
    println!("{result}");
}

fn part2() {
    let time: i64 = 50748685;
    let dist: i64 = 242101716911252;
    let result = quadratic(time as f64, dist as f64);
    println!("{result}");
}

pub fn main() {
    part1();
    part2();
}