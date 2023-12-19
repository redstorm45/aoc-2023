
use std::env;
use std::fs;
use std::collections::HashSet;
use std::cmp::max;


#[derive(Copy,Clone)]
enum Reflector {
    None=0,             // .
    SplitHorizontal=1,  // -
    SplitVertical=2,    // |
    ReflectTopLeft=3,   // /
    ReflectTopRight=4,  // \
}

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
enum Direction {
    Right=0,
    Left=1,
    Bottom=2,
    Top=3,
}

fn parse_symbol(c: &char) -> Reflector {
    match c {
        '-'  => Reflector::SplitHorizontal,
        '|'  => Reflector::SplitVertical,
        '/'  => Reflector::ReflectTopLeft,
        '\\' => Reflector::ReflectTopRight,
        _    => Reflector::None,
    }
}

fn exit_directions(input_dir: &Direction, cell: &Reflector) -> Vec<Direction> {
    /*
    Get the directios when coming into a cell.
    Input direction is reversed (i.e. Right = coming from Left)
    */
    match cell {
        Reflector::None => vec![*input_dir],
        Reflector::SplitHorizontal => {
            match input_dir {
                Direction::Left | Direction::Right => vec![*input_dir],
                _ => vec![Direction::Left, Direction::Right],
            }
        },
        Reflector::SplitVertical => {
            match input_dir {
                Direction::Top | Direction::Bottom => vec![*input_dir],
                _ => vec![Direction::Top, Direction::Bottom],
            }
        },
        Reflector::ReflectTopLeft => {
            match input_dir {
                Direction::Right => vec![Direction::Top],
                Direction::Left => vec![Direction::Bottom],
                Direction::Bottom => vec![Direction::Left],
                Direction::Top => vec![Direction::Right],
            }
        },
        Reflector::ReflectTopRight => {
            match input_dir {
                Direction::Right => vec![Direction::Bottom],
                Direction::Left => vec![Direction::Top],
                Direction::Bottom => vec![Direction::Right],
                Direction::Top => vec![Direction::Left],
            }
        }
    }
}

fn next_coords(x: usize, y:usize, exit_dir:Direction, width:usize, height:usize) -> Option<(usize,usize)> {
    match exit_dir {
        Direction::Top => {
            if x == 0 { None }
            else {Some( ((x as isize-1) as usize, y) )}
        },
        Direction::Left => {
            if y == 0 { None }
            else {Some( (x, (y as isize-1) as usize) )}
        },
        Direction::Bottom => {
            if x+1 == height { None }
            else {Some( (x+1, y) )}
        },
        Direction::Right => {
            if y+1 == width { None }
            else {Some( (x, y+1) )}
        },
    }
}

fn count_energized_from(map:&Vec<Vec<Reflector>>, input_dir:Direction, position: usize) -> usize {
    let width = map.get(0).unwrap().len();

    let initial: (usize,usize,Direction);
    if input_dir == Direction::Right {
        initial = (position, 0, Direction::Right);
    } else if input_dir == Direction::Right {
        initial = (position, width-1, Direction::Left);
    } else {
        initial = (0, position, Direction::Bottom);
    }

    let mut explored: HashSet<(usize,usize,Direction)> = HashSet::new();
    explored.insert( initial );
    let mut heads: Vec<(usize,usize,Direction)> = vec![initial];

    while heads.len() > 0 {
        let cur = heads.pop().unwrap();
        let sym_entered = map.get(cur.0).unwrap().get(cur.1).unwrap();
        for exit_dir in exit_directions(&cur.2, sym_entered) {
            //println!("At ({} {}), entered {} from dir {}, exited {}", cur.0, cur.1, *sym_entered as i32, cur.2 as i32, exit_dir as i32);
            let next = next_coords(cur.0, cur.1, exit_dir, map.len(), map.get(0).unwrap().len());
            if next.is_some() {
                let (x,y) = next.unwrap();
                //println!(" > Move from ({} {}) to ({} {}) on dir {}", cur.0, cur.1, x, y, exit_dir as i32);
                let next_head = (x,y,exit_dir);
                if !explored.contains(&next_head) {
                    heads.push(next_head);
                    explored.insert(next_head);
                } /* else {
                    println!(" x drop");
                }*/
            }
        }
    }

    let mut explored_map: Vec<Vec<bool>> = map.iter().map(|r| r.iter().map(|_| false).collect()).collect();
    for (x,y,_) in explored {
        let v = explored_map.get_mut(x).unwrap().get_mut(y).unwrap();
        *v = true;
    }

    //printblock(&explored_map);

    return explored_map.iter().map(|r|
        r.iter().filter(|v| **v).count()
    ).sum();
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
    
    //let contents = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";

    let map: Vec<Vec<Reflector>> = contents.split('\n').filter(|s| s.len()>0).map(|r| r.chars().map(|c| parse_symbol(&c)).collect()).collect();

    let res1 = count_energized_from(&map, Direction::Right, 0);
    println!("Activated: {res1}");

    let mut res2 = 0;
    for x in 0..(map.len()) {
        res2 = max( res2, count_energized_from(&map, Direction::Left, x) );
        res2 = max( res2, count_energized_from(&map, Direction::Right, x) );
    }
    for y in 0..(map.get(0).unwrap().len()) {
        res2 = max( res2, count_energized_from(&map, Direction::Bottom, y) );
    }
    println!("Max activated: {res2}");
}
