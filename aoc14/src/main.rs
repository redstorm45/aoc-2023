
use std::env;
use std::fs;

fn transpose(v: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<bool>>()
        })
        .collect()
}

fn find_shapes(lines: &Vec<&str>, shape: &char) -> Vec<Vec<bool>> {
    lines.iter().map(|row|
        row.chars().map(|c| c==*shape).collect()
    ).collect()
}

fn roll_line_left(blocks: &Vec<bool>, rolling: &Vec<bool>) -> Vec<bool> {
    let mut min_pos: usize = 0;
    let mut res: Vec<bool> = blocks.iter().map(|_| false).clone().collect();

    for (i, (b,r)) in blocks.iter().zip(rolling).enumerate() {
        if *b {
            min_pos = i+1;
        } else if *r {
            *res.get_mut(min_pos).unwrap() = true;
            min_pos += 1;
        }
    }

    return res;
}

fn roll_platform_west(blocks: &Vec<Vec<bool>>, rolling: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    &blocks.iter()
        .zip(rolling.iter())
        .map(|(b,r)| roll_line_left(b,r))
        .collect()
}

fn roll_platform_up(blocks: &Vec<Vec<bool>>, rolling: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    transpose(
        roll_platform_west(
            transpose(blocks),
            transpose(rolling)
        )
    )
}

fn rotate_platform_cw(m: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // [N,W,S,E] -> [E,S,W,N]
    transpose(m.iter().rev().collect())
}

fn rotate_platform_ccw(m: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // [N,W,S,E] -> [W,S,E,N]
    // [N,W,S,E] ---transpose--> [W,N,E,S] ---mirror U/D--> [W,S,E,N]
    transpose(m).iter().rev().collect()
}

fn roll_platform_round(blocks: &Vec<Vec<bool>>, rolling: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // roll towards N,W,S,E
    let mut r = rotate_platform_ccw(rolling);

    for _ in 0..4 {
        r = rotate_platform_cw( roll_platform_west(r) );
    }

    return rotate_platform_ccw(r);
}

fn platform_weight(rolling: &Vec<Vec<bool>>) -> usize {
    transpose(rolling).iter().map(
        |col| col.iter()
                 .enumerate()
                 .filter(|(_,&x)| x)
                 .map(|(i,_)| col.len()-i)
                 .sum::<usize>()
        ).sum()
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let lines = contents.split('\n').filter(|s| s.len()>0).collect();
    let blocks = find_shapes(&lines, &'#');
    let rolling = find_shapes(&lines, &'O');

    let rolled = roll_platform_up(&blocks, &rolling);

    let w = platform_weight(&rolled);

    println!("Weight: {w}");
}
