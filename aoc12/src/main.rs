
use std::env;
use std::fs;
use memoize::memoize;

#[derive(Debug, Clone)]
enum SpringType {
    Unknown = 0,
    Operational = 1,
    Broken = 2,
}

fn parse_springs(s: &str) -> Vec<SpringType> {
    s.chars().map(|c| match c {
        '?' => SpringType::Unknown,
        '#' => SpringType::Broken,
        _ => SpringType::Operational,
    }).collect()
}

fn parse_springs_map(s: &str) -> (Vec<SpringType>, Vec<i32>) {
    let mut it = s.split(' ');
    let springs = parse_springs(it.next().unwrap());
    let info = it.next().unwrap().split(',').map(|num| num.parse::<i32>().unwrap()).collect();

    return (springs, info);
}

#[memoize]
fn count_spring_possibilities(springs: &Vec<SpringType>, info: &Vec<i32>) -> i32 {
    /*
    Each springset has a set of possible index it could start at (not necessarily contiguous).
    For ease of implementation, use recursion.
    */
    // base case: string ended
    if springs.len() == 0 {
        if info.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    // read first char
    let first = springs.get(0).unwrap();
    if first == SpringType::Operational {
        if info.len() == 0 {
            return 0;
        }
        // try place springset later
        let mut res = count_spring_possibilities(springs[1..].collect(), info);
        // try place springset now
        let first_set_length = info.get(0).unwrap();
        if springs.len() >= first_set_length && springs[..first_set_length].iter().all(|x| )
        return res;
    }

    return 0;
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut res = 0;
    for line in contents.split('\n').filter(|s| s.len()>0) {
        let (springs, info) = parse_springs_map(line);

        res += count_spring_possibilities(springs, info);
    }

    println!("Counted {res} possibilities");
}
