
use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

struct NumInfo {
    xbeg: usize,
    xend: usize, // past the end
    y: usize,
    value: i32
}


fn _print_symbol_ranges(m : &Vec<Vec<bool>>) {
    for line in m {
        for b in line {
            if *b {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}


fn filled_matrix<T : Copy>(height : usize, width : usize, value : T) -> Vec<Vec<T>> {
    let mut new_mat : Vec<Vec<T>> = vec![];
    for _i in 0..height {
        let mut new_line: Vec<T> = vec![];
        for _j in 0..width {
            new_line.push(value);
        }
        new_mat.push(new_line);
    }
    return new_mat;
}

fn extend_square<T: PartialEq + Copy + std::fmt::Display>(mat : Vec<Vec<T>>, empty: T) -> Vec<Vec<T>> {
    /*
    In the matrix, extend each value !=empty so that it is present in a 3x3 square centered on the input point.
    If two squares intersect, fill in reading order.
    */

    // dimensions
    let height = mat.len();
    let width : usize = mat.get(0).unwrap().len();

    // output
    let mut new_mat = filled_matrix::<T>(height, width, empty);

    // go through the input
    for (y,yv) in mat.iter().enumerate() {
        for (x,xv) in yv.iter().enumerate() {
            if *xv != empty {
                // go around
                for i in -1isize..2isize {
                    for j in -1isize..2isize {
                        let x2 = x as isize +j;
                        let y2 = y as isize +i;

                        //print!("({},{})", x2, y2);

                        if y2 >= 0 && y2 < (height as isize) && x2 >= 0 && x2 < (width as isize) {
                            let cell : &mut T = new_mat.get_mut(y2 as usize).unwrap().get_mut(x2 as usize).unwrap();
                            if *cell == empty {
                                *cell = *xv;
                            }
                        }
                    }
                }
            }
        }
    }

    return new_mat;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    // Parse input
    let mut symbol_map: Vec<Vec<bool>> = vec![];
    let mut gear_map: Vec<Vec<i32>> = vec![];
    let mut gear_count: i32 = 0;
    let mut numbers: Vec<NumInfo> = vec![];

    for line in contents.split('\n') {
        if line.len() < 2 {
            continue;
        }
        let mut symbol_line: Vec<bool> = vec![];
        let mut gear_line: Vec<i32> = vec![];
        let mut current_num : String = "".to_string();
        for car in line.chars() {
            match car {
                '0'..='9' => {
                    current_num.push(car);
                },
                _ => {
                    if current_num.len() > 0 {
                        let newnum = NumInfo{
                            xbeg: symbol_line.len() -current_num.len(),
                            xend: symbol_line.len(),
                            y: symbol_map.len(),
                            value: str::parse::<i32>(&current_num).unwrap()
                        };
                        numbers.push(newnum);

                        current_num = "".to_string();
                    }
                },
            }
            match car {
                '0'..='9' | '.' => { symbol_line.push(false); },
                '*' => { symbol_line.push(true); },
                _ => { symbol_line.push(true); },
            }

            if car == '*' {
                gear_count += 1;
                gear_line.push(gear_count);
            } else {
                gear_line.push(0);
            }
        }

        // commit trailing number
        if current_num.len() > 0 {
            let newnum = NumInfo{
                xbeg: symbol_line.len() -current_num.len(),
                xend: symbol_line.len(),
                y: symbol_map.len(),
                value: str::parse::<i32>(&current_num).unwrap()
            };
            numbers.push(newnum);
        }

        symbol_map.push(symbol_line);
        gear_map.push(gear_line);
    }

    // Convolution
    symbol_map = extend_square(symbol_map, false);
    gear_map = extend_square(gear_map, 0);

    // Check the map
    //_print_symbol_ranges(&symbol_map);

    // Get numbers around any symbol
    let mut part_numbers : Vec<i32> = vec![];
    for numinfo in &numbers {
        let mut is_part : bool = false;
        for x in numinfo.xbeg..numinfo.xend {
            if *symbol_map.get(numinfo.y).expect("Y axis").get(x).expect("X axis") {
                is_part = true;
            }
        }
        if is_part {
            part_numbers.push(numinfo.value);
            //println!("Found part number {} at {},{}", numinfo.value, numinfo.xbeg, numinfo.y);
        }
    }

    // Get numbers around gear symbols
    let mut gear_numbers: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (numi,numinfo) in numbers.iter().enumerate() {
        for x in numinfo.xbeg..numinfo.xend {
            let gear_id = *gear_map.get(numinfo.y).expect("Y axis").get(x).expect("X axis");
            if gear_id != 0 {
                let numset = gear_numbers.entry(gear_id).or_insert(HashSet::new());
                numset.insert(numi as i32);
            }
        }
    }

    // Iterate valid gears
    for _,gear_numset in gear_numbers {
        
    }


    let res: i32 = part_numbers.iter().sum();
    println!("Part number total: {res}");
}
