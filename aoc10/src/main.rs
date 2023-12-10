
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

enum Direction {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut map: Vec<Vec<PipeType>> = vec![];
    let mut search_heads: Vec<> = vec![];
    for (i,line) in contents.split('\n').enumerate() {
        if line.len() < 2 {
            continue;
        }

        for (j,c) in line.chars() {
            if c == 'S' {
                search_heads = vec![
                    (i, j, Direction::Right),
                    (i, j, Direction::Up),
                    (i, j, Direction::Left),
                    (i, j, Direction::Down),
                ];
            }
        }
        let j = line.chars().find(|x| x=='S');

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
}
