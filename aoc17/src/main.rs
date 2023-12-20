
use std::env;
use std::fs;
use std::hash::Hash;
use std::collections::HashMap;


#[derive(Copy,Clone,Eq,PartialEq,Hash,Debug)]
enum Direction {
    Right=0,
    Left=1,
    Bottom=2,
    Top=3,
}

#[derive(Copy,Clone,Eq,PartialEq,Hash,Debug)]
struct SearchState {
    x: usize,
    y: usize,
    entry_dir: Direction,
    transition_count: usize,
}

pub trait Inserted<K: Eq + Hash, V> {
    fn inserted(&mut self, item: K, value: V) -> bool;
}

impl<K: Eq + Hash, V> Inserted<K, V> for HashMap<K, V> {
    fn inserted(&mut self, item: K, value: V) -> bool {
        return match self.entry(item) {
            std::collections::hash_map::Entry::Occupied(o) => false,
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(value);
                true
            },
        };
    }
}

struct SearchHead {
    state: SearchState,
    total_score: usize,
}

fn exit_possibilities(entry_dir:Direction, needs_turn:bool) -> Vec<Direction> {
    if !needs_turn {
        match entry_dir {
            Direction::Right => vec![Direction::Right, Direction::Bottom, Direction::Top],
            Direction::Left => vec![Direction::Left, Direction::Bottom, Direction::Top],
            Direction::Bottom => vec![Direction::Left, Direction::Right, Direction::Bottom],
            Direction::Top => vec![Direction::Left, Direction::Right, Direction::Top],
        }
    } else {
        match entry_dir {
            Direction::Right => vec![Direction::Bottom, Direction::Top],
            Direction::Left => vec![Direction::Bottom, Direction::Top],
            Direction::Bottom => vec![Direction::Left, Direction::Right],
            Direction::Top => vec![Direction::Left, Direction::Right],
        }
    }
}

fn opposite(dir:Direction) -> Direction {
    match dir {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Top => Direction::Bottom,
        Direction::Bottom => Direction::Top,
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

fn _printscores(block: &Vec<Vec<i32>>) {
    for lineb in block.iter() {
        for b in lineb.iter() {
            let c = (('0' as u8) as i32 + *b) as u8 as char;
            print!("{}", c);
        }
        println!("");
    }
}

fn _printblock(block: &Vec<Vec<bool>>) {
    for lineb in block.iter() {
        for b in lineb.iter() {
            if *b {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    /*
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");
    */

    let contents = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";

    let map: Vec<Vec<i32>> = contents.split('\n').filter(|s| s.len()>0).map(|r| r.chars().map(|c| c as i32 - '0' as i32).collect()).collect();

    _printscores(&map);

    fn get_map_at(map:&Vec<Vec<i32>>, x:usize, y:usize) -> i32 {
        if x!=0 && y!= 0 {
            *map.get(y).unwrap().get(x).unwrap()
        } else {
            0
        }
    }

    const MAX_TRANSITIONS: usize = 3;

    let mut explored: HashMap<SearchState, Option<SearchState>> = HashMap::new();
    for dir in [Direction::Left,Direction::Right,Direction::Top,Direction::Bottom] {
        explored.insert(SearchState{
            x:0,
            y:0,
            entry_dir:dir,
            transition_count:0
        }, None);
    }

    // heads: cell and entry direction
    let mut heads: Vec<SearchHead> = vec![
        SearchHead{state:SearchState{x:0, y:0, entry_dir:Direction::Right, transition_count:0}, total_score:0},
        SearchHead{state:SearchState{x:0, y:0, entry_dir:Direction::Bottom, transition_count:0}, total_score:0}
    ];
    let (height,width) = (map.len(), map.get(0).unwrap().len());

    let mut solution: Option<SearchState> = None;
    while heads.len() > 0 {
        // pop lowest distance first (explore breadth-first)
        heads.sort_by(|a,b| a.total_score.cmp(&b.total_score).reverse());
        let current = heads.pop().unwrap();

        // early-exit
        if current.state.x+1 == height && current.state.y+1 == width {
            println!("Solution found at score {}", current.total_score);
            solution = Some(current.state);
            break;
        }

        // map all possibilitie from 'current'
        for exit_dir in exit_possibilities(current.state.entry_dir, current.state.transition_count+1>=MAX_TRANSITIONS) {
            let next_maybe = next_coords(current.state.x, current.state.y, exit_dir, width, height);
            if next_maybe.is_some() {
                // we can get to 'next' from 'current'.
                // compute a new Head which does that transition
                let next = next_maybe.unwrap();
                let new_score = current.total_score + (get_map_at(&map, next.0, next.1) as usize);
                let mut new_transition_count = 0;
                if current.state.entry_dir==exit_dir {
                    new_transition_count = current.state.transition_count+1;
                }
                let new_state = SearchState{
                    x: next.0,
                    y: next.1,
                    entry_dir: exit_dir,
                    transition_count: new_transition_count,
                };

                if explored.inserted(new_state, Some(current.state)) {
                    heads.push(SearchHead{
                        state: new_state,
                        total_score:new_score,
                    });
                }
            }
        }
    }

    assert!(solution.is_some(), "Solution was not found");

    let mut solution_map: Vec<Vec<bool>> = map.iter().map(|r| r.iter().map(|_| false).collect()).collect();

    //dbg!(&explored);

    let mut path: Vec<(usize,usize)> = vec![(height-1, width-1)];
    let mut prev_opt: Option<SearchState> = solution;
    while prev_opt.is_some() {
        let prev = prev_opt.unwrap();
        path.push( (prev.x, prev.y) );
        *solution_map.get_mut(prev.y).unwrap().get_mut(prev.x).unwrap() = true;
        let prev_prev = explored.get( &prev ).unwrap();
        if prev_prev.is_some() {
            let pp = prev_prev.unwrap();
            //println!("Previous of {} {} is {} {}", prev.x, prev.y, pp.x, pp.y);
        }
        prev_opt = *prev_prev;
    }

    _printblock(&solution_map);

    dbg!(path);
}
