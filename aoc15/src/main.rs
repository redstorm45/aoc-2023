
use std::env;
use std::fs;

fn hash(s: &str) -> u64 {
    let mut res = 0;
    for c in s.chars() {
        res += (c as u8) as u64;
        res *= 17;
        res %= 256;
    }
    return res;
}

fn split_symbol(s: &str) -> (String, char, u8) {
    match s.chars().last().unwrap() {
        '-' => (s[..(s.len()-1)].to_string(), '-', 0),
        c => (s[..(s.len()-2)].to_string(), '=', (c as u8)-('0' as u8)),
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");
    //let contents = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

    // first hash check
    let mut res = 0;
    for sym in contents.split('\n').next().unwrap().split(',') {
        res += hash(sym);
    }
    println!("Total hashed: {res}");

    // apply box operations
    let mut boxes: [Vec<(String,u8)>;256] = vec![Vec::new();256].try_into().expect("static");
    for sym in contents.split('\n').next().unwrap().split(',') {
        let (label, operation, focal) = split_symbol(sym);
        let target_box = hash(&label);
        let box_vec = boxes.get_mut(target_box as usize).unwrap();
        let index = box_vec.iter().position(|x| x.0==label);
        if operation == '-' {
            if index.is_some() {
                box_vec.remove(index.unwrap());
            }
        } else {
            if index.is_some() {
                *box_vec.get_mut(index.unwrap()).unwrap() = (label, focal);
            } else {
                box_vec.push((label, focal));
            }
        }
    }

    // compute lens hash
    let mut power = 0;
    for (i,boxvec) in boxes.iter().enumerate() {
        for (j, (_,lens)) in boxvec.iter().enumerate() {
            power += (i+1) * (j+1) * (*lens as usize);
        }
    }
    println!("Total power: {power}");
}
