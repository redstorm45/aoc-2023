
use std::env;
use std::fs;

fn parse_intseq(s : &str) -> Vec<i64> {
    return s.split(' ').filter(|&x| x.len()>0).map(|x| x.parse::<i64>().expect("Not a number")).collect();
}

fn next_val(v : &Vec<i64>) -> i64 {
    let mut diff: Vec<i64> = vec![];
    let mut prev: Option<i64> = None;
    for num in v {
        if !prev.is_none() {
            diff.push( *num - prev.unwrap() );
        }
        prev = Some(*num);
    }

    if diff.iter().all(|&x| x==0) {
        return prev.unwrap();
    } else {
        let additional = next_val(&diff);
        return prev.unwrap() + additional;
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut res : i64 = 0;
    let mut res2 : i64 = 0;

    for line in contents.split('\n') {
        if line.len() < 2 {
            continue;
        }
        let seq = parse_intseq(line);

        res += next_val(&seq);
        res2 += next_val(&seq.into_iter().rev().collect());
    }

    println!("Result end: {res}");
    println!("Result start: {res2}");
}
