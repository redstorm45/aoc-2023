
use std::env;
use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct MapSegment {
    beg : i64,
    end : i64,
    offset : i64
}


fn parse_intseq(s : &str) -> Vec<i64> {
    return s.split(' ').filter(|x| x.len()>0).map(|x| x.parse::<i64>().expect("Not a number")).collect();
}

fn get_after_map(x : &i64, map : &Vec<MapSegment>) -> i64 {
    for seg in map {
        if x >= &seg.beg && x < &seg.end {
            return x + &seg.offset;
        }
    }
    return *x;
}

fn get_after_map_range(range: &(i64,i64), map : &Vec<MapSegment>) -> Vec<(i64,i64)> {
    let mut res : Vec<(i64,i64)> = vec![];
    let mut to_map : Vec<(i64,i64)> = vec![ *range ];

    for map_seg in map {
        let mut new_to_map : Vec<(i64,i64)> = vec![];

        for (beg,len) in to_map {
            if beg >= map_seg.end || beg+len < map_seg.beg {
                // disjoint segments
                new_to_map.push( (beg,len) );
            } else {
                // intersecting segments
                let mut mapped_beg = beg;
                if beg < map_seg.beg {
                    // bit at start is unmapped
                    new_to_map.push( (beg,map_seg.beg-beg) );
                    mapped_beg = map_seg.beg;
                }
                let mut mapped_end = beg+len;
                if beg+len >= map_seg.end {
                    // bit at end is unmapped
                    new_to_map.push( (map_seg.end,beg+len-map_seg.end) );
                    mapped_end = map_seg.end;
                }
                // mapped part
                res.push( (mapped_beg+map_seg.offset,mapped_end-mapped_beg) )
            }
        }

        to_map = new_to_map;
    }

    // remaining unmapped
    for (beg,len) in to_map {
        res.push( (beg,len) );
    }

    return res;
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut lines_iter = contents.split('\n');

    let mut seeds : Vec<i64> = parse_intseq( lines_iter.next().unwrap().split(':').next_back().unwrap() );
    let mut seed_ranges : Vec<(i64,i64)> = vec![];
    for (a,b) in seeds.iter().tuples() {
        seed_ranges.push( (*a,*b) );
    }

    //dbg!(&seeds);
    //dbg!(&seed_ranges);

    let mut current_map : Vec<MapSegment> = vec![];

    let mut has_next_map = true;
    lines_iter.next();
    while has_next_map {

        let map_info = lines_iter.next(); // drop the "<>-to-<> map:" line

        // read the map
        loop {
            let line = lines_iter.next();
            if line.is_none() {
                has_next_map = false;
                break;
            }
            else if line.unwrap().len() == 0 {
                break;
            } else {
                let mappings = parse_intseq(line.unwrap());

                let [begb, bega, len]: [_;3] = mappings.try_into().unwrap();

                current_map.push(MapSegment{
                    beg: bega,
                    end: bega+len,
                    offset: begb-bega
                })
            }
        }

        // apply the map
        seeds = seeds.iter().map(|x| get_after_map(x, &current_map)).collect();
        seed_ranges = seed_ranges.iter().map(|r| get_after_map_range(r, &current_map)).collect::<Vec<Vec<(i64,i64)>>>().concat();

        //dbg!(&current_map);

        current_map = vec![];

        //dbg!(&seeds);
        //dbg!(&seed_ranges);
    }

    seeds.sort();
    seed_ranges.sort_by(|(a1,_b1), (a2,_b2)| a1.cmp(a2) );

    let first_seed = seeds.get(0).unwrap();
    let (first_seed2, _) = seed_ranges.get(0).unwrap();

    println!("Lowest location (part1) is {first_seed}");
    println!("Lowest location (part2) is {first_seed2}");
}
