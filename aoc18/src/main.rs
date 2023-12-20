
use std::env;
use std::fs;
use std::ops::Range;


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

#[derive(Clone)]
struct CompressedVec<T> {
    data: Vec<(T,usize)>,
}

impl<T: Clone> CompressedVec<T> {
    fn default() -> CompressedVec<T> {
        CompressedVec{data:vec![]}
    }

    fn push_multi(&mut self, val: T, count: usize) {
        self.data.push( (val,count) );
    }

    fn get_compressed_index(&self, index: usize) -> (usize,usize) {
        // get the index of the segment and the observed index of the start of that segment
        // such that the segment contains the asked 'index'
        let mut current_index: usize = 0;
        let mut segment_index: usize = self.data.len();
        for (i,(_,seg_size)) in self.data.iter().enumerate() {
            if current_index <= index && index < current_index + seg_size {
                segment_index = i;
                break;
            }
            current_index += seg_size;
        }
        return (segment_index, current_index);
    }

    fn get_ranges(&mut self, beg: usize, end:usize) -> Range<usize> {
        // get a reference to a set of blocks that exactly cover the range [beg,end[
        // split segments as needed

        //println!("Get ranges [{beg},{end}[");

        // get and split at beginning
        let (seg_index_1,idx1) = self.get_compressed_index(beg);
        let mut seg_beg_index = seg_index_1;
        if idx1 != beg {
            //println!("Splitting blockidx {} starting at {} of len {}, cut before {} (beg)", seg_beg_index, idx1, self.data.get(seg_beg_index).unwrap().1, beg);
            // need to create a new block
            let value = self.data.get_mut(seg_beg_index).unwrap().0.clone();
            let newblock_len = beg-idx1;
            self.data.get_mut(seg_beg_index).unwrap().1 -= newblock_len;
            self.data.insert(seg_beg_index, (value, newblock_len));
            seg_beg_index += 1;
        }

        // get and split at end
        let (seg_index_2,idx2) = self.get_compressed_index(end);
        let mut seg_end_index = seg_index_2;
        if idx2 != end {
            //println!("Splitting blockidx {} starting at {} of len {}, cut before {} (end)", seg_end_index, idx2, self.data.get(seg_end_index).unwrap().1, end);
            // need to create a new block
            let value = self.data.get(seg_end_index).unwrap().0.clone();
            let newblock_len = end-idx2;
            self.data.get_mut(seg_end_index).unwrap().1 -= newblock_len;
            self.data.insert(seg_end_index, (value, newblock_len));
            seg_end_index += 1;
        }

        return seg_beg_index..seg_end_index;
    }

    fn set_range(&mut self, beg: usize, end:usize, value: T) {
        let range = self.get_ranges(beg, end);
        for (val,_) in self.data[range].iter_mut() {
            *val = value.clone();
        }
    }
}

struct PipeMap {
    data: CompressedVec<CompressedVec<PipeType>>,
}

impl PipeMap {
    fn empty(width: usize, height: usize) -> PipeMap {
        //println!("Create map of size {width} {height}");
        let mut line: CompressedVec<PipeType> = CompressedVec::default();
        line.push_multi(PipeType::None, width);

        let mut col: CompressedVec<CompressedVec<PipeType>> = CompressedVec::default();
        col.push_multi(line, height);

        return PipeMap{data:col};
    }

    fn get(&mut self, x: usize, y: usize) -> Option<&mut PipeType> {
        // get single cell at x,y
        //println!("Get y={} line", y);
        let ry = self.data.get_ranges(y, y+1).start;
        let (line,_) = self.data.data.get_mut(ry)?;
        //println!("Get x={} cell", x);
        let rx = line.get_ranges(x, x+1).start;
        let (cell,_) = line.data.get_mut(rx)?;
        return Some(cell);
    }

    fn set_range(&mut self, beg: (usize,usize), end: (usize,usize), value: PipeType) {
        // set a rectangle [beg,end[ to 'value'
        let ranges_y = self.data.get_ranges(beg.1, end.1);
        for ry in ranges_y {
            let (line,_) = self.data.data.get_mut(ry).unwrap();
            line.set_range(beg.0, end.0, value);
            /*
            let ranges_x = line.get_ranges(beg.0, end.0);
            for rx in ranges_x {
                line.data.get_mut(rx).unwrap().0 = value;
            }
            */
        }
    }
}

fn opposite_dir(c: char) -> char {
    match c {
        'U' => 'D',
        'L' => 'R',
        'R' => 'L',
        _ => 'U',
    }
}


fn _printblock(block: &PipeMap) {
    for (line, line_height) in block.data.data.iter().rev() {
        for _ in 0..*line_height {
            for (b,symlen) in line.data.iter() {
                for _ in 0..*symlen {
                    let c = match *b {
                        PipeType::None => ".",
                        PipeType::Horizontal => "-",
                        PipeType::Vertical => "|",
                        PipeType::BendUL => "J",
                        PipeType::BendUR => "L",
                        PipeType::BendDL => "7",
                        PipeType::BendDR => "F",
                    };
                    print!("{}", c);
                }
            }
            println!("");
        }
    }
}


fn get_dig_area(instructions: &Vec<(char,usize)>) -> usize {
    let mut mini: (isize,isize) = (0,0);
    let mut maxi: (isize,isize) = (0,0);
    {
        let mut current: (isize,isize) = (0,0);
        for (dir,len) in instructions.iter() {
            current = match dir {
                'L' => (current.0 - (*len as isize),current.1),
                'R' => (current.0 + (*len as isize),current.1),
                'U' => (current.0, current.1 + (*len as isize)),
                _ => (current.0, current.1 - (*len as isize)),
            };

            if current.0 < mini.0 {
                mini.0 = current.0;
            }
            if current.1 < mini.1 {
                mini.1 = current.1;
            }
            if current.0 > maxi.0 {
                maxi.0 = current.0;
            }
            if current.1 > maxi.1 {
                maxi.1 = current.1;
            }
        }
    }

    //println!("Found ranges: {} {}, {} {}", mini.0, mini.1, maxi.0, maxi.1);

    let mut map: PipeMap = PipeMap::empty( (maxi.0-mini.0+1) as usize, (maxi.1-mini.1+1) as usize);

    let mut current: (usize,usize) = (-mini.0 as usize, -mini.1 as usize);
    let mut prev_dir: Option<char> = None;
    for (dir,len) in instructions.iter() {
        //println!("Instruction: {} for {}", dir, len);
        if *dir == 'L' {
            if prev_dir.is_some() {
                *map.get(current.0,current.1).unwrap() = match opposite_dir(prev_dir.unwrap()) {
                    'U' => PipeType::BendUL,
                    'D' => PipeType::BendDL,
                    _ => PipeType::None,
                };
            }
            if *len > 1 {
                map.set_range((current.0-*len+1,current.1), (current.0,current.1+1), PipeType::Horizontal);
            }
            current = (current.0 - len,current.1);
        } else if *dir == 'R' {
            if prev_dir.is_some() {
                *map.get(current.0,current.1).unwrap() = match opposite_dir(prev_dir.unwrap()) {
                    'U' => PipeType::BendUR,
                    'D' => PipeType::BendDR,
                    _ => PipeType::None,
                };
            }
            if *len > 1 {
                map.set_range((current.0+1,current.1), (current.0+*len,current.1+1), PipeType::Horizontal);
            }
            current = (current.0 + len,current.1);
        } else if *dir == 'U' {
            if prev_dir.is_some() {
                *map.get(current.0,current.1).unwrap() = match opposite_dir(prev_dir.unwrap()) {
                    'L' => PipeType::BendUL,
                    'R' => PipeType::BendUR,
                    _ => PipeType::None,
                };
            }
            if *len > 1 {
                map.set_range((current.0,current.1+1), (current.0+1,current.1+*len), PipeType::Vertical);
            }
            current = (current.0,current.1 + len);
        } else {
            if prev_dir.is_some() {
                *map.get(current.0,current.1).unwrap() = match opposite_dir(prev_dir.unwrap()) {
                    'L' => PipeType::BendDL,
                    'R' => PipeType::BendDR,
                    _ => PipeType::None,
                };
            }
            if *len > 1 {
                map.set_range((current.0,current.1-*len+1), (current.0+1,current.1), PipeType::Vertical);
            }
            current = (current.0,current.1 - len);
        }
        prev_dir = Some(*dir);
        //_printblock(&map);
        //println!("");
    }

    *map.get(current.0,current.1).unwrap() = match (instructions.get(0).unwrap().0, opposite_dir(prev_dir.unwrap())) {
        ('L', 'U') => PipeType::BendUL,
        ('U', 'L') => PipeType::BendUL,
        ('R', 'U') => PipeType::BendUR,
        ('U', 'R') => PipeType::BendUR,
        ('L', 'D') => PipeType::BendDL,
        ('D', 'L') => PipeType::BendDL,
        ('D', 'R') => PipeType::BendDR,
        ('R', 'D') => PipeType::BendDR,
        _ => PipeType::None,
    };

    //_printblock(&map);

    let mut area = 0;
    for (line, line_height) in map.data.data.iter() {
        let mut in_top = false; // is the top right corner in area
        let mut in_bot = false; // is the bottom right corner in area
        for (pipe, pipe_width) in line.data.iter() {
            if *pipe != PipeType::None {
                if *pipe == PipeType::Vertical {
                    in_top = !in_top;
                    in_bot = !in_bot;
                } else if *pipe == PipeType::Horizontal {
                    // don't change anything
                } else if *pipe == PipeType::BendUR {
                    // no wall -> on wall
                    in_top = !in_top;
                } else if *pipe == PipeType::BendDR {
                    // no wall -> on wall
                    in_bot = !in_bot;
                } else if *pipe == PipeType::BendUL {
                    // on wall -> no wall
                    in_top = !in_top;
                } else if *pipe == PipeType::BendDL {
                    // on wall -> no wall
                    in_bot = !in_bot;
                }
                area += line_height*pipe_width;
            } else {
                if in_top { // !is_wall -> (in_top == in_bot)
                    area += line_height*pipe_width;
                }
            }
        }
    }

    return area;
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    /*
    let contents = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    */

    let mut instructions: Vec<(char,usize)> = vec![];
    let mut instructions2: Vec<(char,usize)> = vec![];

    for line in contents.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let mut line_it = line.split(' ');

        {
            let direction = line_it.next().unwrap().chars().next().unwrap();
            let len = line_it.next().unwrap().parse::<usize>().unwrap();
            instructions.push( (direction,len) );
        }

        {
            let hexa = &line_it.next().unwrap()[2..8];
            let direction = match hexa.chars().rev().next().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                _ => 'U',
            };
            let len = usize::from_str_radix(&hexa[..5], 16).unwrap();
            instructions2.push( (direction,len) );
        }
    }


    let area = get_dig_area(&instructions);
    let area2 = get_dig_area(&instructions2);
    println!("Found dig area: {area}");
    println!("Found real dig area: {area2}");
}
