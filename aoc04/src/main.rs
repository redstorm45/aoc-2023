
use std::env;
use std::fs;
use std::collections::VecDeque;
use num_bigint::BigUint;
use num_bigint::ToBigUint;


fn parse_integers(text: &str) -> Vec<i32> {
    return text.split(' ')
               .filter(|s| s.len() > 0)
               .map(|s| str::parse::<i32>(s).unwrap())
               .collect();
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut card_queue : VecDeque<BigUint> = VecDeque::from([]);

    let mut res1 : i32 = 0;
    let mut res2 : BigUint = num_traits::Zero::zero();

    for line in contents.split('\n') {
        if line.len() < 2 {
            continue;
        }
        let mut parts = line.split(':').next_back().expect("No data part").split('|');
        let card = parse_integers(parts.next().expect("No card data"));
        let scratched = parse_integers(parts.next().expect("No scratch data"));

        fn zeropow(x : i32) -> i32 {
            if x == 0 {
                return 0;
            } else {
                return 2_i32.pow((x-1) as u32);
            }
        }
        let match_count = scratched.iter().filter(|x| card.contains(x)).count().try_into().unwrap();
        res1 += zeropow(match_count);

        let current_mul : BigUint = card_queue.pop_front().unwrap_or(num_traits::Zero::zero()) +1 as u32;
        println!("Adding {current_mul} cards to next {match_count}");
        res2 += current_mul.clone();

        for i in 0..match_count {
            if card_queue.len() <= i.try_into().unwrap() {
                card_queue.push_back(current_mul.clone());
            } else {
                *card_queue.get_mut(i as usize).unwrap() += current_mul.clone();
            }
        }
    }

    println!("Total score: {res1}");
    println!("Total cards: {res2}");
}
