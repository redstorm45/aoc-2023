
use std::env;
use std::fs;


#[derive(Copy, Clone, Eq, PartialEq)]
enum PipeType {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
    BendUL = 3,
    BendUR = 4,
    BendDL = 5,
    BendDR = 6,
}

#[derive(Copy, Clone)]
enum Direction {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

fn reverse_direction(dir: &Direction) -> Direction {
    return match dir {
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
        Direction::Down => Direction::Up,
        Direction::Up => Direction::Down,
    }
}

fn convert_pipe(dir: &Direction, pipe: &PipeType) -> Option<Direction> {
    return match pipe {
        PipeType::None => None,
        PipeType::Horizontal => {
            match dir {
                Direction::Right => Some(Direction::Right),
                Direction::Left => Some(Direction::Left),
                _ => None,
            }
        },
        PipeType::Vertical => {
            match dir {
                Direction::Up => Some(Direction::Up),
                Direction::Down => Some(Direction::Down),
                _ => None,
            }
        },
        PipeType::BendUL => {
            match dir {
                Direction::Right => Some(Direction::Up),
                Direction::Down => Some(Direction::Left),
                _ => None,
            }
        },
        PipeType::BendDL => {
            match dir {
                Direction::Right => Some(Direction::Down),
                Direction::Up => Some(Direction::Left),
                _ => None,
            }
        },
        PipeType::BendUR => {
            match dir {
                Direction::Left => Some(Direction::Up),
                Direction::Down => Some(Direction::Right),
                _ => None,
            }
        },
        PipeType::BendDR => {
            match dir {
                Direction::Left => Some(Direction::Down),
                Direction::Up => Some(Direction::Right),
                _ => None,
            }
        },
    }
}

fn get_pipe(dir1: &Direction, dir2: &Direction) -> PipeType {
    // get pipe from two outgoing directions
    return match (dir1,dir2) {
        (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => PipeType::Vertical,
        (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => PipeType::Horizontal,
        (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => PipeType::BendUR,
        (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => PipeType::BendUL,
        (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => PipeType::BendDR,
        (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => PipeType::BendDL,
        _ => PipeType::None,
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut map: Vec<Vec<PipeType>> = vec![];
    let mut search_heads: Vec<(usize,usize,Direction)> = vec![];
    for (i,line) in contents.split('\n').enumerate() {
        let ic = i as usize;
        if line.len() < 2 {
            continue;
        }

        for (j,c) in line.chars().enumerate() {
            let jc = j as usize;
            if c == 'S' {
                search_heads = vec![
                    (ic, jc, Direction::Right),
                    (ic, jc, Direction::Up),
                    (ic, jc, Direction::Left),
                    (ic, jc, Direction::Down),
                ];
            }
        }

        map.push(
            line.chars().map(|x| match x {
                '-' => PipeType::Horizontal,
                '|' => PipeType::Vertical,
                'J' => PipeType::BendUL,
                'L' => PipeType::BendUR,
                '7' => PipeType::BendDL,
                'F' => PipeType::BendDR,
                _ => PipeType::None,
            }).collect()
        );
    }

    let mut loop_tiles: Vec<(usize,usize)> = vec![];
    for (hx, hy, hd) in search_heads.iter() {
        let (mut cx, mut cy, mut cd) = (*hx as isize, *hy as isize, *hd);
        loop_tiles = vec![(*hx,*hy)];
        let mut found = false;

        while loop_tiles.len() ==1 || (cx!=(*hx as isize) || cy!=(*hy as isize)) {
            loop_tiles.push( (cx as usize,cy as usize) );

            let (nx, ny) = match cd {
                Direction::Right => (cx, cy+1),
                Direction::Left  => (cx, cy-1),
                Direction::Down  => (cx+1, cy),
                Direction::Up    => (cx-1, cy),
            };

            if nx < 0 || nx >= (map.len() as isize) || ny < 0 || ny >= (map.get(0).unwrap().len() as isize) {
                //println!("Search head ended on border {} {} {} {}", nx, ny, map.len(), map.get(0).unwrap().len());
                break;
            }

            if (nx,ny) == (*hx as isize,*hy as isize) {
                let start_pipe = get_pipe(hd, &reverse_direction(&cd));
                *map.get_mut(*hx).unwrap().get_mut(*hy).unwrap() = start_pipe;
                //println!("Overwritting start pipe {}", start_pipe as i32);
                found = true;
                break;
            }

            let p = map.get(nx as usize).unwrap().get(ny as usize).unwrap();
            let nd = convert_pipe(&cd, p);

            if nd.is_none() {
                //println!("Search head ended on nothing");
                break;
            }

            (cx, cy, cd) = (nx, ny, nd.unwrap());
        }

        if found {
            break;
        }
    }

    let mut walls: Vec<Vec<bool>> = vec![];
    for line in &map {
        walls.push(line.iter().map(|_| false).collect());
    }

    for (i,j) in loop_tiles.iter() {
        *walls.get_mut(*i).unwrap().get_mut(*j).unwrap() = true;
    }

    /*
    for (i,wall) in walls.iter().enumerate() {
        let mut in_top = false; // is the top right corner in area
        let mut in_bot = false; // is the bottom right corner in area
        for (j,w) in wall.iter().enumerate() {
            if *w {
                let pipe = *map.get(i).unwrap().get(j).unwrap();
                if pipe == PipeType::Vertical {
                    in_top = !in_top;
                    in_bot = !in_bot;
                } else if pipe == PipeType::Horizontal {
                    // don't change anything
                } else if pipe == PipeType::BendUR {
                    // no wall -> on wall
                    in_top = !in_top;
                } else if pipe == PipeType::BendDR {
                    // no wall -> on wall
                    in_bot = !in_bot;
                } else if pipe == PipeType::BendUL {
                    // on wall -> no wall
                    in_top = !in_top;
                } else if pipe == PipeType::BendDL {
                    // on wall -> no wall
                    in_bot = !in_bot;
                }
            }

            if *w {
                print!("O");
            } else if in_top {
                print!("*");
            } else {
                print!(".");
            }

            if in_top && in_bot {
                print!("=");
            } else if in_top {
                print!("-");
            } else if in_bot {
                print!("_");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    */

    let mut area = 0;
    for (i,line) in walls.iter().enumerate() {
        let mut in_top = false; // is the top right corner in area
        let mut in_bot = false; // is the bottom right corner in area
        for (j,is_wall) in line.iter().enumerate() {
            if *is_wall {
                let pipe = *map.get(i).unwrap().get(j).unwrap();
                if pipe == PipeType::Vertical {
                    in_top = !in_top;
                    in_bot = !in_bot;
                } else if pipe == PipeType::Horizontal {
                    // don't change anything
                } else if pipe == PipeType::BendUR {
                    // no wall -> on wall
                    in_top = !in_top;
                } else if pipe == PipeType::BendDR {
                    // no wall -> on wall
                    in_bot = !in_bot;
                } else if pipe == PipeType::BendUL {
                    // on wall -> no wall
                    in_top = !in_top;
                } else if pipe == PipeType::BendDL {
                    // on wall -> no wall
                    in_bot = !in_bot;
                }
            } else {
                if in_top { // !is_wall -> (in_top == in_bot)
                    area += 1;
                }
            }
        }
    }

    println!("Max dist is {}", loop_tiles.len()/2);
    println!("Area is {}", area);
}
