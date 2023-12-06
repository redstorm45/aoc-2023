
use std::env;
use std::fs;
use std::iter;

struct NumInfo {
    xbeg: usize,
    xend: usize, // past the end
    y: usize,
    value: i32
}

/*
fn print_symbol_ranges(m : Vec<Vec<bool>>) {
    for line in m {
        for b in line {
            if b {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    // Parse input
    let mut symbol_map: Vec<Vec<bool>> = vec![];
    let mut numbers: Vec<NumInfo> = vec![];

    for line in contents.split('\n') {
        if line.len() < 2 {
            continue;
        }
        let mut symbol_line: Vec<bool> = vec![];
        let mut current_num : String = "".to_string();
        for car in line.chars() {
            match car {
                '0'..='9' => {
                    current_num.push(car);
                },
                _ => {
                    if current_num.len() > 0 {
                        let length = current_num.len();

                        let newnum = NumInfo{
                            xbeg: symbol_line.len() -length,
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
                _ => { symbol_line.push(true); },
            }
        }
        symbol_map.push(symbol_line);
    }

    // Convolution L/R
    for line in symbol_map.iter_mut() {
        let mut newline: Vec<bool> = vec![];

        for i in 0..line.len() {
            let mut is_close: bool = *line.get(i).expect("");
            if i > 1 {
                is_close = is_close || *line.get(i-1).expect("");
            }
            if i < line.len()-1 {
                is_close = is_close || *line.get(i+1).expect("");
            }
            newline.push(is_close);
        }
        *line = newline;
    }

    // Convolution U/D
    fn zip_or(a : &Vec<bool>, b : &Vec<bool>) -> Vec<bool> {
        return iter::zip(a,b).map( |(a,b)| *a||*b ).collect();
    }

    {
        let mut new_map : Vec<Vec<bool>> = vec![];
        for i in 0..symbol_map.len() {
            let mut newline : Vec<bool> = symbol_map.get(i).expect("").to_vec();
            if i > 1 {
                newline = zip_or(&newline, symbol_map.get(i-1).expect(""))
            }
            if i < symbol_map.len()-1 {
                newline = zip_or(&newline, symbol_map.get(i+1).expect(""))
            }
            new_map.push(newline);
        }
        symbol_map = new_map;
    }

    // Check the map
    //print_symbol_ranges(symbol_map);

    // Eliminate numbers not close to symbols
    let mut part_numbers : Vec<i32> = vec![];
    for numinfo in numbers {
        let mut is_part : bool = false;
        for x in numinfo.xbeg..numinfo.xend {
            if *symbol_map.get(numinfo.y).expect("Y axis").get(x).expect("X axis") {
                is_part = true;
            }
        }
        if is_part {
            part_numbers.push(numinfo.value);
        }
    }


    let res: i32 = part_numbers.iter().sum();
    println!("Part number total: {res}");
}
