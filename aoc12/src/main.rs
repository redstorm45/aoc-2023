
use std::env;
use std::fs;
use memoize::memoize;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

fn repeated_join(s: &str, count:&usize, joiner: &str) -> String {
    let mut res = (*s).to_owned();
    for _ in 1..*count {
        res += joiner;
        res += s;
    }
    return res;
}

fn parse_springs_map(s: &str, repeat: &Option<usize>) -> (Vec<SpringType>, Vec<i32>) {
    let repeat_count = repeat.unwrap_or(1);

    let mut it = s.split(' ');
    let springs_str = it.next().unwrap();
    let springs = parse_springs( &repeated_join(springs_str, &repeat_count, "?") );
    let info_str = it.next().unwrap();
    let info = repeated_join(info_str, &repeat_count, ",").split(',').map(|num| num.parse::<i32>().unwrap()).collect();

    return (springs, info);
}

fn can_place_springset(springs: &Vec<SpringType>, length: &usize) -> bool {
    springs.len() >= *length && springs[..*length].iter().all(|&x| x != SpringType::Operational)
}

#[memoize]
fn count_spring_possibilities(springs: Vec<SpringType>, info: Vec<i32>) -> i64 {
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
    let first = *springs.get(0).unwrap();
    if first == SpringType::Broken {
        if info.len() == 0 {
            return 0;
        }
        let mut res = 0;
        // try place springset now
        let first_set_length = *info.get(0).unwrap() as usize;
        if can_place_springset(&springs, &first_set_length) {
            if springs.len() == first_set_length {
                // place until end
                if info.len() == 1 {
                    res += 1;
                }
            } else {
                // leave one operational between sets
                if *springs.get(first_set_length).unwrap() != SpringType::Broken {
                    res += count_spring_possibilities(springs[(first_set_length+1)..].iter().copied().collect(), info[1..].iter().copied().collect());
                }
            }
        }
        return res;
    } else if first == SpringType::Unknown {
        let mut res = 0;
        // try place springset later
        res += count_spring_possibilities(springs[1..].iter().copied().collect(), info.clone());
        // try place springset now
        if info.len() > 0 {
            let first_set_length = *info.get(0).unwrap() as usize;
            if can_place_springset(&springs, &first_set_length) {
                if springs.len() == first_set_length {
                    // place until end
                    if info.len() == 1 {
                        res += 1;
                    }
                } else {
                    // leave one operational between sets
                    if *springs.get(first_set_length).unwrap() != SpringType::Broken {
                        res += count_spring_possibilities(springs[(first_set_length+1)..].iter().copied().collect(), info[1..].iter().copied().collect());
                    }
                }
            }
        }
        return res;
    } else {
        // place springs later
        return count_spring_possibilities(springs[1..].iter().copied().collect(), info);
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut res = 0;
    let mut res2 = 0;
    for line in contents.split('\n').filter(|s| s.len()>0) {
        let (springs, info) = parse_springs_map(line, &None);
        res += count_spring_possibilities(springs, info);

        let (springs2, info2) = parse_springs_map(line, &Some(5));
        res2 += count_spring_possibilities(springs2, info2);
    }

    println!("Counted {res} possibilities");
    println!("Counted {res2} second possibilities");
}
