
use std::env;
use std::fs;


fn reflection_indexes(v: &Vec<i64>) -> Vec<usize> {
    if v.len() < 2 {
        return vec![];
    }

    return (1..v.len())
        .filter(|&i| v[..i].iter()
            .rev()
            .zip(v[i..].iter())
            .all(|(a,b)| a==b))
        .collect()
}

fn count_high_bits(x: &i64) -> i64{
    let mut res = 0;
    let mut val = *x;
    while val > 0 {
        res += val%2;
        val /= 2;
    }
    return res;
}

fn reflection_indexes_smudged(v: &Vec<i64>) -> Vec<usize> {
    if v.len() < 2 {
        return vec![];
    }

    let refl = reflection_indexes(v);

    return (1..v.len())
        .filter(|i| !refl.contains(i))
        .filter(|&i| v[..i].iter()
            .rev()
            .zip(v[i..].iter())
            .map(|(a,b)| count_high_bits(&(a^b)))
            .sum::<i64>() == 1)
        .collect()
}

fn transpose(v: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect()
        })
        .collect()
}

fn line_to_int(v: &Vec<bool>) -> i64 {
    let mut res = 0;
    for e in v {
        if *e {
            res = (res*2) +1;
        } else {
            res *= 2;
        }
    }
    return res;
}

fn block_score(block: &Vec<Vec<bool>>) -> usize {
    let mut res = 0;

    res += reflection_indexes(&transpose(block).iter().map(line_to_int).collect()).iter().sum::<usize>();
    res += 100*( reflection_indexes(&block.iter().map(line_to_int).collect()).iter().sum::<usize>() );

    return res;
}

fn block_score_smudged(block: &Vec<Vec<bool>>) -> usize {
    let mut res = 0;

    res += reflection_indexes_smudged(&transpose(block).iter().map(line_to_int).collect()).iter().sum::<usize>();
    res += 100*( reflection_indexes_smudged(&block.iter().map(line_to_int).collect()).iter().sum::<usize>() );

    return res;
}

/*
fn printblock(block: &Vec<Vec<bool>>) {
    for line in block {
        for c in line {
            if *c {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
*/

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");
    /*
    let contents = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.\n
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#\n";
    */

    let mut res = 0;
    let mut res2 = 0;
    let mut block: Vec<Vec<bool>> = vec![];
    for line in contents.split('\n') {
        if line.len() == 0 {
            if block.len() == 0 {
                continue;
            }

            //printblock(&block);
            //println!("");

            res += block_score(&block);
            res2 += block_score_smudged(&block);
            block = vec![];
        } else {
            block.push( line.chars().map(|c| c=='#').collect() );
        }
    }

    println!("Total: {res}");
    println!("Total2: {res2}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection_indexes_empty() {
        assert_eq!(reflection_indexes(&vec![]), vec![]);
    }

    #[test]
    fn test_reflection_indexes_sym() {
        assert_eq!(reflection_indexes(&vec![2, 2]), vec![1]);
    }

    #[test]
    fn test_reflection_indexes_offleft() {
        assert_eq!(reflection_indexes(&vec![4, 5, 2, 2]), vec![3]);
    }

    #[test]
    fn test_reflection_indexes_offright() {
        assert_eq!(reflection_indexes(&vec![1, 2, 2, 1, 3, 5]), vec![2]);
    }

    #[test]
    fn test_reflection_indexes_smudged_empty() {
        assert_eq!(reflection_indexes_smudged(&vec![]), vec![]);
    }

    #[test]
    fn test_reflection_indexes_smudged_sym() {
        assert_eq!(reflection_indexes_smudged(&vec![2, 3]), vec![1]);
    }

    #[test]
    fn test_reflection_indexes_smudged_offleft() {
        assert_eq!(reflection_indexes_smudged(&vec![2, 5, 2, 6]), vec![3]);
    }

    #[test]
    fn test_reflection_indexes_smudged_offright() {
        assert_eq!(reflection_indexes_smudged(&vec![1, 7, 3, 1, 3, 5]), vec![2]);
    }
}
