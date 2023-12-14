use std::env;
use std::fs;
//use std::string;
//use std::iter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let strdigits = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut res : u32 = 0;
    let mut res2 : u32 = 0;
    let mut lastletters = "".to_owned();

    for line in contents.split('\n') {
        let mut first : Option<char> = None;
        let mut last : Option<char> = None;

        let mut first_digit : Option<char> = None;
        let mut last_digit : Option<char> = None;

        for cr in line.chars() {
            match cr {
                '0'..='9' => {
                    if first.is_none() {
                        first = Some(cr);
                    }
                    if first_digit.is_none() {
                        first_digit = Some(cr);
                    }
                    last = Some(cr);
                    last_digit = Some(cr);
                },
                'a'..='z' => {
                    lastletters.push(cr);

                    let mut digmatch : Option<char> = None;

                    for (val,dig) in strdigits.iter().enumerate() {
                        if dig.len() > lastletters.len() {
                            continue;
                        }
                        let mut matched = true;
                        for i in 0..dig.len() {
                            if dig.as_bytes()[i] != lastletters.as_bytes()[lastletters.len()-dig.len()+i] {
                                matched = false;
                            }
                        }
                        if matched {
                            digmatch = char::from_digit(val.try_into().unwrap(), 10);
                        }
                    }

                    if digmatch.is_some() {
                        if first.is_none() {
                            first = digmatch;
                        }
                        last = digmatch;
                    }
                },
                _ => {}
            }
        }

        match (first_digit,last_digit) {
            (Some(x),Some(y)) => {
                res += x.to_digit(10).unwrap()*10 + y.to_digit(10).unwrap();
            },
            (_,_) => {}
        }
        match (first,last) {
            (Some(x),Some(y)) => {
                res2 += x.to_digit(10).unwrap()*10 + y.to_digit(10).unwrap();
            },
            (_,_) => {}
        }
    }

    println!("Result: {res}");
    println!("Result 2: {res2}");
}
