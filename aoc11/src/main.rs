
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

fn empty_row_indexes(v: &Vec<Vec<bool>>) -> Vec<usize> {
    v.iter().enumerate().filter(|(_,r)| r.iter().filter(|&k| *k).count()==0).map(|(i,_)| i).collect()
}

fn expand_positions(v: &Vec<(usize,usize)>, xgrow: &Vec<usize>, ygrow: &Vec<usize>, amount:usize) -> Vec<(usize,usize)> {
    // replace cells at indicated indexes by cells of size 'amount'
    v.iter().map(|(x,y)| (
        x + xgrow.iter().filter(|&v| v<x).count()*(amount-1),
        y + ygrow.iter().filter(|&v| v<y).count()*(amount-1),
    )).collect()
}

fn get_positions(map: &Vec<Vec<bool>>) -> Vec<(usize,usize)> {
    let mut res: Vec<(usize,usize)> = vec![];
    for (i,row) in map.iter().enumerate() {
        for (j,v) in row.iter().enumerate() {
            if *v {
                res.push( (i,j) );
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
    /*
    let map: Vec<Vec<bool>> = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#....."
                    .split('\n').filter(|s| s.len()>0)
                                      .map(|s| s.chars().map(|c| c=='#').collect())
                                      .collect();
    */

    let galaxies = get_positions(&map);
    //dbg!(&galaxies);
    let xempty = empty_row_indexes(&map);
    let yempty = empty_row_indexes(&transpose(map));

    //dbg!(&xempty);
    //dbg!(&yempty);

    for grow in [2,1000000] {
        let positions = expand_positions(&galaxies, &xempty, &yempty, grow);

        //dbg!(&positions);

        let mut res = 0;
        for ((ax,ay), (bx,by)) in iter_pairs(positions.iter().map(|(x,y)| (*x as isize, *y as isize)).collect()) {
            res += (ax-bx).abs() + (ay-by).abs();
        }

        println!("total dist {res}");
    }
}
