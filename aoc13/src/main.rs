
use std::env;
use std::fs;


fn reflection_indexes(v: &Vec<i64>) -> Vec<usize> {

}

fn line_to_int(v: &Vec<bool>) -> i64 {
    let mut res = 0;
    for e in v {
        if e {
            res = (res*2) +1;
        } else {
            res *= 2;
        }
    }
    return res;
}

fn block_score(block: &Vec<Vec<bool>>) -> i32 {
    let res = 0;

    return res;
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut res = 0;
    let mut block: Vec<Vec<bool>> = vec![];
    for line in contents.split('\n') {
        if line.len() == 0 {
            if block.len() == 0 {
                continue;
            }

            res += block_score(&block);
            block = vec![];
        } else {
            block.push( line.chars().map(|c| c=='#').collect() );
        }
    }

    println!("Total: {res}");
}
