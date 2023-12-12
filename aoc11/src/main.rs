
use std::env;
use std::fs;


fn transpose(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<bool>>()
        })
        .collect()
}

fn grow_once(map: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut res: Vec<Vec<bool>> = vec![];
    for row in map {
        if row.iter().filter(|x| **x).count() == 0 {
            res.push(row.clone());
        }
        res.push(row);
    }
    return res;
}

fn grow_universe(map: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    grow_once( transpose(grow_once(transpose(map))) )
}

fn get_positions(map: Vec<Vec<bool>>) -> Vec<(i32,i32)> {
    let mut res: Vec<(i32,i32)> = vec![];
    for (i,row) in map.iter().enumerate() {
        for (j,v) in row.iter().enumerate() {
            if *v {
                res.push( (i as i32,j as i32) );
            }
        }
    }
    return res;
}

fn iter_pairs<T: Copy>(v: Vec<T>) -> Vec<(T,T)> {
    // [0,1,2] -> (0,1), (0,2), (1,2)
    return v.iter().enumerate().flat_map(|(i,a)| v[(i+1)..].iter().map(|b| (*a,*b))).collect();
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let map: Vec<Vec<bool>> = contents.split('\n').filter(|s| s.len()>0)
                                      .map(|s| s.chars().map(|c| c=='#').collect())
                                      .collect();

    let expanded = grow_universe(map);
    let galaxies = get_positions(expanded);

    let mut res = 0;
    for ((ax,ay), (bx,by)) in iter_pairs(galaxies) {
        res += (ax-bx).abs() + (ay-by).abs();
    }

    println!("total dist {res}");
}
