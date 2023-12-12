
use std::env;
use std::fs;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_traits::identities::{Zero, One};


fn to_node_id(s : &str) -> i64 {
    let mut res : i64 = 0;
    for c in s.chars() {
        match c {
            'A'..='Z' => {
                res *= 100;
                res += (c as i64) - ('A' as i64);
            },
            _ => {}
        }
    }
    return res;
}

fn side_to_bools(s : &str) -> Vec<bool> {
    return s.chars().map(|c| c=='R').collect();
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    // find gcd(a,b) and numbers s and t such that sa+tb = gcd(a,b)
    // returns (gcd(a,b), s, t)
    let (mut oldr,mut  r) = (a.clone(),b.clone());
    let (mut olds,mut  s) = (BigInt::one(),BigInt::zero());
    let (mut oldt,mut  t) = (BigInt::zero(),BigInt::one());


    while r != BigInt::zero() {
        let quotient = oldr.clone() / r.clone();
        (oldr, r) = (r.clone(), oldr - quotient.clone()*r);
        (olds, s) = (s.clone(), olds - quotient.clone()*s);
        (oldt, t) = (t.clone(), oldt - quotient*t);
    }

    //println!("gcd ({a}, {b}) = {oldr}");

    return (oldr, olds, oldt);
}

fn modular_inverse(a: BigInt, n: BigInt) -> Option<BigInt> {
    // get inverse of a modulo n
    // i.e. satisfies ax = 1 mod n
    let (g, x, _) = extended_gcd(&a, &n);
    if g == BigInt::one() {
        return Some(x);
    }
    return None;
}

fn positive_mod(a: BigInt,b: BigInt) -> BigInt{
    let r = a%b.clone();
    if r < BigInt::zero() {
        return r+b;
    }
    return r;
}

// ex: (B=5, A=4)<>(B=6, A=6) -> impossible
//     (B=1, A=3)<>(B=0, A=5) -> (B=10, A=15)
// offset is C such that C + k*lcm(A,A') = Ai+B = A'j+B'
// it should be the first value with that property, with i>=0 and j>=0
// A'i-Aj = B-B'
// ex : 5i - 3j = 1 - 0    (i=2, j=3)
fn merge_loops(loop1: (BigInt,BigInt), loop2: (BigInt,BigInt)) -> (BigInt,BigInt) {
    let (start1, size1) = loop1;
    let (start2, size2) = loop2;

    println!("Combine loops {start1}+k{size1} and {start2}+k{size2}");

    let (gcd,_,_) = extended_gcd(&size1, &size2);
    let size = size1.clone()*size2.clone()/gcd.clone();

    println!("Found loop size {size}");

    // S = A + ka*A' = B+kb*B'
    // A-B + ka*A' = kb*B'
    // define A" = A'/gcd(A',B')              (used so that modular inverse works: A" and B' should be relatively prime)
    // (A-B + ka*A') mod A" = kb*B' mod A"
    // A-B mod A" = kb*B' mod A"
    // define X = inverse of B' mod A"
    // (A-B)*X mod A" = kb

    /*
    let small_size2 = size2.clone()/gcd.clone();
    let small_size1 = size1.clone()/gcd;

    let inv1 = modular_inverse(size1, small_size2);
    let inv2 = modular_inverse(size2.clone(), small_size1).unwrap();

    let idx1 = positive_mod((size1.clone()-size2.clone())*inv2, small_size2);
    let idx2 = positive_mod((size2-size1)*inv1, small_size1);

    let start = start2.clone() + idx1.clone()*size2.clone();
    */
    let small_size1 = size1.clone() / gcd;
    let inv = modular_inverse(size2.clone(), small_size1.clone()).unwrap();
    let idx = positive_mod( (start1.clone()-start2.clone())*inv, small_size1 );
    let start = start2.clone() + idx.clone()*size2.clone();

    println!("Found size {} and starting value {} = {} + {}*{} = {} + {}*k",
             size, start,
             start2, size2, idx,
             start1, size1);
    return (start, size);
}

fn tuple_indexed(pair : &(i64,i64), idx : bool) -> i64 {
    if idx {
        return pair.1;
    } else {
        return pair.0;
    }
}

fn get_loop_description(map: &HashMap<i64,(i64,i64)>, dir: &Vec<bool>, start: i64) -> (i64, Vec<i64>, Vec<i64>) {
    /*
    Since map and dir are finite, the path will end up in a loop,
    of maximum size |map|*|dir| = 702*271 = 190242 and minimum size |dir| = 271

    This loop will have a size S, and a number of 'Z' nodes
    The length to reach the Z nodes in that loop is always C_i + k*S

    This function returns S, followed by all C_i, followed by all indexes of Z nodes before the loop
    */
    let mut visited: HashMap<(i64,i64),i64> = HashMap::new(); // map (node_id, dir_idx) -> path_idx
    let mut path_length = 0; // for readability, but is = to visited.len()
    let mut current = (start,0);
    let mut z_indexes: Vec<i64> = vec![];

    while !visited.contains_key(&current) {
        visited.insert(current, path_length);

        if current.0%100 == 25 {
            z_indexes.push(path_length);
        }

        let dir_idx = (path_length as usize)%dir.len();
        path_length +=1;
        let next_dir_idx = (path_length as usize)%dir.len();

        //println!("Visit {}, going to {}", &current.0, *dir.get(dir_idx).unwrap());

        // follow instructions, store next cell & next index
        let curcell = tuple_indexed(map.get(&current.0).unwrap(),
                                    *dir.get(dir_idx).unwrap());
        current = (curcell, next_dir_idx as i64);
    }
    
    //dbg!(&visited);

    let loop_path_length = *visited.get(&current).unwrap(); // size of path before the loop
    let size = path_length - loop_path_length; // size of the loop

    let z_before_loop = z_indexes.iter().filter(|&len| len<&loop_path_length).map(|&x| x).collect();
    let z_in_loop = z_indexes.iter().filter(|&len| len>=&loop_path_length).map(|&x| x).rev().collect();

    return (size, z_in_loop, z_before_loop);
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");
    //let contents = "LR\n\nAAA = (BBB, XXX)\nBBB = (XXX, ZZZ)\nZZZ = (BBB, XXX)\nCCA = (CCB, XXX)\nCCB = (CCC, CCC)\nCCC = (CCZ, CCZ)\nCCZ = (CCB, CCB)\nXXX = (XXX, XXX)";
    //let contents = "LR\n\nAAA = (BBB, XXX)\nBBB = (XXX, ZZZ)\nZZZ = (BBB, XXX)\nCCA = (CCB, XXX)\nCCB = (CCC, CCC)\nCCC = (CCZ, CCZ)\nCCZ = (CCB, CCB)\nXXX = (XXX, XXX)";

    let mut contents_it = contents.split('\n');

    let sides = side_to_bools( contents_it.next().unwrap() );
    contents_it.next();

    let mut directions: HashMap<i64,(i64,i64)> = HashMap::new();
    for line in contents_it {
        if line.len() < 2 {
            continue;
        }
        let mut line_split = line.split('=');
        let source = to_node_id( &line_split.next().unwrap() );

        let mut dir_split = line_split.next().unwrap().split(',');
        let dir_left = to_node_id( &dir_split.next().unwrap() );
        let dir_right = to_node_id( &dir_split.next().unwrap() );

        directions.insert(source, (dir_left,dir_right));
    }

    //dbg!(&sides);
    //dbg!(&directions);

    let mut res = 0;
    let mut current_node = 0;
    let target_node = to_node_id("ZZZ");
    while current_node != target_node {
        let side_to_right = *sides.get( res % sides.len() ).unwrap();

        current_node = tuple_indexed(directions.get(&current_node).unwrap(), side_to_right);
        res += 1;
    }
    println!("Reached end in {} steps", res);

    /*
    // bruteforce does not work, too long

    let mut res2 = 0;
    let mut current_nodes: Vec<i64> = directions.keys().filter(|k| (*k)%26 == 0).map(|&x| x).collect();

    println!("Using {} paths at the same time", current_nodes.len());

    while current_nodes.iter().any(|x| x%26 != 25) {
        let side_to_right = *sides.get( res2 % sides.len() ).unwrap();

        for node in current_nodes.iter_mut() {
            *node = tuple_indexed(directions.get(node).unwrap(), side_to_right);
        }
        res2 += 1;

        if res2%10000 == 0 {
            println!("... Step {}", res2);
        }
    }

    println!("Reached end (2) in {} steps", res2);
    */
    let mut start_nodes: Vec<i64> = directions.keys().filter(|k| (*k)%100 == 0).map(|&x| x).collect();
    start_nodes.sort();
    let mut curloop: Option<(BigInt,BigInt)> = None; // (C,S) to have C + k*S

    for start in start_nodes {
        println!("Start at {start}");
        let desc = get_loop_description(&directions, &sides, start);

        println!("Found description starting at {}", start);
        dbg!(&desc);

        // simplification: only one Z node in the loop, none outside (valid on input data)
        let newloop = (*desc.1.get(0).unwrap(), desc.0);
        let big_newloop = (BigInt::from(newloop.0), BigInt::from(newloop.1));


        if curloop.is_none() {
            curloop = Some(big_newloop);
        } else {
            curloop = Some( merge_loops(curloop.unwrap(), big_newloop) );
        }
    }

    println!("Global loop starts at {}", curloop.unwrap().0)
}
