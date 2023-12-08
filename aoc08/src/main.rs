
use std::env;
use std::fs;
use std::collections::HashMap;


fn to_node_id(s : &str) -> i32 {
    let mut res : i32 = 0;
    for c in s.chars() {
        match c {
            'A'..='Z' => {
                res *= 26;
                res += (c as i32) - ('A' as i32);
            },
            _ => {}
        }
    }
    return res;
}

fn side_to_bools(s : &str) -> Vec<bool> {
    return s.chars().map(|c| c=='R').collect();
}

fn extended_gcd(a: i32, b:i32) -> (i32, i32, i32) {
    // find gcd(a,b) and numbers s and t such that sa+tb = gcd(a,b)
    // returns (gcd(a,b), s, t)
    let mut (oldr, r) = (a,b);
    let mut (olds, s) = (1,0);
    let mut (oldt, t) = (0,1);

    while r != 0 {
        let quotient = old_r % r;
        (oldr, r) = (r, oldr - quotient*r);
        (olds, s) = (s, olds - quotient*s);
        (oldt, t) = (t, oldt - quotient*t);
    }

    return (oldr, olds, oldt);
}

fn tuple_indexed(pair : &(i32,i32), idx : bool) -> i32 {
    if idx {
        return pair.1;
    } else {
        return pair.0;
    }
}

fn get_loop_description(map: &HashMap<i32,(i32,i32)>, dir: &Vec<bool>, start: i32) -> (i32, Vec<i32>, Vec<i32>) {
    /*
    Since map and dir are finite, the path will end up in a loop,
    of maximum size |map|*|dir| = 702*271 = 190242 and minimum size |dir| = 271

    This loop will have a size S, and a number of 'Z' nodes
    The length to reach the Z nodes in that loop is always C_i + k*S

    This function returns S, followed by all C_i, followed by all indexes of Z nodes before the loop
    */
    let mut visited: HashMap<(i32,i32),i32> = HashMap::new(); // map (node_id, dir_idx) -> path_idx
    let mut path_length = 0; // for readability, but is = to visited.len()
    let mut current = (start,0);
    let mut z_indexes: Vec<i32> = vec![];

    while !visited.contains_key(&current) {
        visited.insert(current, path_length);

        if current.0%26 == 25 {
            z_indexes.push(path_length);
        }

        path_length +=1;
        let dir_idx = (path_length as usize)%dir.len();

        current = (tuple_indexed(map.get(&current.0).unwrap(), *dir.get(dir_idx).unwrap()), dir_idx as i32);
    }

    let loop_path_length = *visited.get(&current).unwrap(); // size of path before the loop
    let size = path_length - loop_path_length; // size of the loop

    let z_before_loop = z_indexes.iter().filter(|&len| len<&loop_path_length).map(|&x| x).collect();
    let z_in_loop = z_indexes.iter().filter(|&len| len>=&loop_path_length).map(|&x| x).collect();

    return (size, z_in_loop, z_before_loop);
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut contents_it = contents.split('\n');

    let sides = side_to_bools( contents_it.next().unwrap() );
    contents_it.next();

    let mut directions: HashMap<i32,(i32,i32)> = HashMap::new();
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
    let mut current_nodes: Vec<i32> = directions.keys().filter(|k| (*k)%26 == 0).map(|&x| x).collect();

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
    let mut start_nodes: Vec<i32> = directions.keys().filter(|k| (*k)%26 == 0).map(|&x| x).collect();
    let mut curloop: Option<(i32,i32)> = None; // (C,S) to have C + k*S

    for start in start_nodes {
        let desc = get_loop_description(&directions, &sides, start);

        // simplification: only one Z node in the loop, none outside (valid on input data)
        let newloop = (desc.1.get(0).unwrap(), desc.0);

        if curloop.is_none() {
            curloop = newloop;
        } else {
            newsize = num::integer::lcm(curloop.0, newloop.0);
            // ex: (B=5, A=4)<>(B=6, A=6) -> impossible
            //     (B=1, A=3)<>(B=0, A=5) -> (B=10, A=15)
            // offset is C such that C + k*lcm(A,A') = Ai+B = A'j+B'
            // it should be the first value with that property, with i>=0 and j>=0
            // A'i-Aj = B-B'
            // ex : 5i - 3j = 1 - 0    (i=2, j=3)
            
        }

        println!("Found description starting at {}", start);
        dbg!(desc);
    }
}
