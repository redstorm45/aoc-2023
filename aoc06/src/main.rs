
use std::env;
use std::fs;
use std::iter::zip;


fn parse_intseq(s : &str) -> Vec<i64> {
    return s.split(' ').filter(|x| x.len()>0).map(|x| x.parse::<i64>().expect("Not a number")).collect();
}

fn parse_int_with_whitespaces(s : &str) -> i64 {
    return s.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<i64>().expect("Not a number");
}

fn parse_both(line : &str) -> (Vec<i64>, i64) {
    let numbers = line.split(":").skip(1).next().unwrap();

    return (parse_intseq(numbers), parse_int_with_whitespaces(numbers));
}

fn win_count(duration : i64, distance : i64) -> i64 {
    // with t=time to press, T=duration, D=distance record, d=achieved distance
    // dist(t) = (T-t)*t = -t² + Tt
    // dist > D <=> -t² + Tt > D <=> -t² +Tt -D > 0 <=> t² -Tt +D < 0
    // Delta = T²-4D
    // R = (T +- sqrt(Delta))/2

    let det = (duration*duration - 4*distance) as f64;
    let r1 = (duration as f64 - det.sqrt())/2.0;
    let r2 = (duration as f64 + det.sqrt())/2.0;

    // if R1 < t < R2, then dist(t) > D
    // t integer => (R1 < t < R2  <=> floor(R1) < t < ceil(R2))
    // R1,R2 never integer => (_ <=> ceil(R1) <= t <= floor(R2))
    // count = floor(R2) - ceil(R1) +1

    let count = (r2.floor() - r1.ceil()) as i64 +1;

    return count;
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut contents_it = contents.split('\n');

    let time_line = contents_it.next().expect("No time line");
    let distance_line = contents_it.next().expect("No distance line");

    let (times, bigtime) = parse_both(time_line);
    let (distances, bigdist) = parse_both(distance_line);

    let mut res: i64 = 1;

    for (duration,distance) in zip(times,distances) {
        res *= win_count(duration, distance);
    }

    let resbig = win_count(bigtime, bigdist);

    println!("Solution 1: {res}");

    println!("Solution 2: {resbig}");
}
