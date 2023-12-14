
use std::env;
use std::fs;
use std::collections::HashMap;

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
    blocks.iter()
        .zip(rolling.iter())
        .map(|(b,r)| roll_line_left(b,r))
        .collect()
}

fn roll_platform_up(blocks: &Vec<Vec<bool>>, rolling: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    transpose(
        &roll_platform_west(
            &transpose(&blocks),
            &transpose(&rolling)
        )
    )
}

fn rotate_platform_cw(m: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // [N,W,S,E] -> [E,S,W,N]
    let rev: Vec<Vec<bool>> = m.iter().rev().cloned().collect();
    transpose(&rev)
}

fn rotate_platform_ccw(m: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // [N,W,S,E] -> [W,S,E,N]
    // [N,W,S,E] ---transpose--> [W,N,E,S] ---mirror U/D--> [W,S,E,N]
    transpose(m).into_iter().rev().collect()
}

fn roll_platform_round(blocks: &Vec<Vec<bool>>, rolling: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // roll towards N,W,S,E
    /*
    println!("source");
    printblock(&blocks,&rolling);
    */

    let mut r = rotate_platform_ccw(&rolling);
    let mut b = rotate_platform_ccw(&blocks);

    /*
    println!("CCW");
    printblock(&b,&r);
    */

    for _ in 0..4 {
        r = rotate_platform_cw( &roll_platform_west(&b, &r) );
        b = rotate_platform_cw(&b);
        /*
        println!("CW");
        printblock(&b,&r);
        */
    }

    let res = rotate_platform_cw(&r);
    /*
    println!("CW");
    printblock(&b,&r);
    println!("-------");
    */
    return res;
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

fn printblock(block: &Vec<Vec<bool>>, roll: &Vec<Vec<bool>>) {
    for (lineb, liner) in block.iter().zip(roll.iter()) {
        for (b,r) in lineb.iter().zip(liner.iter()) {
            if *b {
                print!("#");
            } else if *r {
                print!("O");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    //let contents = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

    let lines = contents.split('\n').filter(|s| s.len()>0).collect();
    let blocks = find_shapes(&lines, &'#');
    let rolling = find_shapes(&lines, &'O');

    let rolledup = roll_platform_up(&blocks, &rolling);

    let w = platform_weight(&rolledup);
    println!("Weight: {w}");

    let mut known: HashMap<Vec<Vec<bool>>,i32> = HashMap::new();
    let mut current: Vec<Vec<bool>> = rolling;
    let mut step = 0;
    while !known.contains_key(&current) {
        known.insert(current.clone(), step);
        current = roll_platform_round(&blocks, &current);
        step += 1;
    }
    let mut loopstep = known.get(&current).unwrap();
    // loop happens from 'loop' to 'loopstep'
    // go to cycle before the end
    let remaining_steps = (1000000000-loopstep)%(step-loopstep);
    for i in 0..remaining_steps {
        current = roll_platform_round(&blocks, &current);
    }

    let w2 = platform_weight(&current);
    println!("Weight 2: {w2}");
    // 1345 too low
}
