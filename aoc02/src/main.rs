use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let bagsize = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut res = 0;
    let mut res2 = 0;

    for line in contents.split('\n') {
        let mut isvalid = true;

        if line.len() < 2{
            continue;
        }

        println!("Lines: {line}");

        let gameid = line.split(':').next().expect("First part of string").split(' ').last().expect("Game id").parse::<u32>().unwrap();
        let hands = line.split(':').last().expect("Second part of string");
        let mut miniset = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);

        for hand in hands.split(';') {
            for part in hand.split(',') {
                let set = &part.split(' ').collect::<Vec<&str>>()[1..3];
                let count = str::parse::<i32>(set[0]).unwrap();
                let color = set[1];

                *miniset.get_mut(color).unwrap() = cmp::max(*miniset.get(color).expect("Unknown color"), count);

                let maxi = *bagsize.get(color).expect("Count of cubes");

                if count > maxi {
                    isvalid = false;
                }
            }
        }

        let gamesum = miniset.get("red").expect("No red") * miniset.get("green").expect("No green") * miniset.get("blue").expect("No blue");

        res2 += gamesum;
        if isvalid {
            res += gameid;
        }
    }

    println!("Result 1: {res}");
    println!("Result 2: {res2}");
}
