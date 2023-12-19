
use std::env;
use std::fs;
use std::collections::HashMap;


#[derive(Debug,PartialEq)]
enum Operator {
    IsGreaterThan,
    IsLessThan,
}

fn varname_to_index(c: &char) -> usize {
    match c {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        _   => 3, //s
    }
}

fn symbol_to_operator(c: &char) -> Operator {
    match c {
        '<' => Operator::IsLessThan,
        _ => Operator::IsGreaterThan,
    }
}

#[derive(Debug)]
struct Step {
    variable: usize,
    operator: Operator,
    constant: usize,
    dest_flow: String,
}

#[derive(Debug)]
struct Workflow {
    steps: Vec<Step>,
    fallback: String,
}

fn parse_workflow(s: &str) -> (String, Workflow) {
    let name = s.split('{').next().unwrap();
    let inside = &s[name.len()+1..s.len()-1];

    let mut steps:Vec<Step> = vec![];
    let mut lastname: String = "".to_string();
    for part in inside.split(',') {
        if part.contains(':') {
            steps.push(Step{
                variable: varname_to_index(&part.chars().next().unwrap()),
                operator: symbol_to_operator(&part[1..].chars().next().unwrap()),
                constant: part.split(':').next().unwrap()[2..].parse::<usize>().unwrap(),
                dest_flow: part.split(':').last().unwrap().to_string(),
            });
        } else {
            lastname = part.to_string();
        }
    }

    let wf = Workflow{
        steps: steps,
        fallback: lastname,
    };

    return (name.to_string(), wf);
}

fn is_part_accepted(part: &[usize;4], flows: &HashMap<String,Workflow>) -> bool {
    let mut current_flow: String = "in".to_string();

    while current_flow != "R" && current_flow != "A" {
        let flow = flows.get(&current_flow).unwrap();
        let mut moved: bool = false;
        for step in &flow.steps {
            if step.operator == Operator::IsLessThan && part[step.variable] < step.constant {
                moved = true;
                current_flow = step.dest_flow.clone();
                break;
            } else if step.operator == Operator::IsGreaterThan && part[step.variable] > step.constant {
                moved = true;
                current_flow = step.dest_flow.clone();
                break;
            }
        }

        if !moved {
            current_flow = flow.fallback.clone();
        }
    }

    return current_flow == "A";
}

#[derive(Debug,Copy,Clone)]
struct PartRange {
    offset: [usize;4],
    size: [usize;4],
}

fn with_replaced_range(range: &PartRange, variable: &usize, offset: usize, size: usize) -> PartRange {
    let mut new_offset: [usize;4] = range.offset;
    let mut new_size: [usize;4] = range.size;

    new_offset[*variable] = offset;
    new_size[*variable] = size;

    return PartRange{
        offset: new_offset,
        size: new_size,
    }
}

fn split_range(range: &PartRange, variable: &usize, constant: &usize, operator: &Operator) -> (Option<PartRange>, Option<PartRange>) {
    // split a block into a (valid range, invalid range)
    let target_offset = range.offset[*variable];
    let target_size = range.size[*variable];

    if *operator == Operator::IsLessThan {
        // check [Off, Off+Size[ < Cst
        if target_offset+target_size < *constant {
            return (Some(*range), None);
        } else if *constant <= target_offset {
            return (None, Some(*range));
        }

        return (
            Some(with_replaced_range(range, variable, target_offset, *constant-target_offset)),
            Some(with_replaced_range(range, variable, *constant, (target_offset+target_size)-constant))
        )
    } else {
        // check [Off, Off+Size[ > Cst
        if target_offset+target_size <= *constant {
            return (None, Some(*range));
        } else if *constant < target_offset {
            return (Some(*range), None);
        }

        return (
            Some(with_replaced_range(range, variable, *constant+1, (target_offset+target_size)-*constant-1)),
            Some(with_replaced_range(range, variable, target_offset, *constant-target_offset+1))
        )
    }
}

fn resolve_range(flows: &HashMap<String,Workflow>) -> Vec<PartRange> {
    let mut ranges: HashMap<String,Vec<PartRange>> = HashMap::new();
    let mut success: Vec<PartRange> = vec![];

    const FULL_RANGE: PartRange = PartRange{offset:[1,1,1,1], size:[4000,4000,4000,4000]};
    ranges.insert("in".to_string(), vec![FULL_RANGE]);
    while ranges.keys().len() > 0 {
        // pop first
        let first_key: String = ranges.keys().next().unwrap().clone();
        let (flowname, waiting_ranges) = ranges.remove_entry(&first_key).unwrap();

        //println!("Applying flow {}", &flowname);
        //dbg!(&waiting_ranges);

        // apply the flow
        let mut after_flow: Vec<(String,PartRange)> = vec![];
        for to_sort in waiting_ranges {
            let mut remaining = Some(to_sort);
            let flow = flows.get(&flowname).unwrap();
            let mut i_opt = None;
            for step in &flow.steps {
                let valid_invalid = split_range(&remaining.unwrap(), &step.variable, &step.constant, &step.operator);
                let v_opt = valid_invalid.0;
                i_opt = valid_invalid.1;
                if v_opt.is_some() {
                    after_flow.push( (step.dest_flow.clone(),v_opt.unwrap()) );
                }
                remaining = i_opt;
                if i_opt.is_none() {
                    break;
                }
            }
            if i_opt.is_some() {
                after_flow.push( (flow.fallback.clone(),i_opt.unwrap()) );
            }
        }

        //println!("Result:");
        //dbg!(&after_flow);

        // insert the results
        for (flowname, to_add) in after_flow {
            if flowname == "A" {
                success.push(to_add);
            } else if flowname == "R" {
                //drop
            } else {
                ranges.entry(flowname).or_insert(vec![]).push(to_add);
            }
        }
    }

    return success;
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    /*
    let contents="px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    */

    let mut contents_it = contents.split('\n');

    let mut workflows: HashMap<String,Workflow> = HashMap::new();
    loop {
        let line = contents_it.next().unwrap();

        if line.len() == 0 {
            break;
        }

        let (name,wf) = parse_workflow(line);

        workflows.insert(name, wf);
    }

    //dbg!(&workflows);

    let mut parts: Vec<[usize;4]> = vec![];
    for line in contents_it {
        if line.len() == 0 {
            continue;
        }

        let mut partvar: [usize;4] = [0, 0, 0, 0];

        for var in line[1..(line.len()-1)].split(',') {
            partvar[varname_to_index(&var.split('=').next().unwrap().chars().next().unwrap())] = var.split('=').last().unwrap().parse::<usize>().unwrap();
        }

        parts.push(partvar);
    }

    //dbg!(&parts);

    let mut res: usize = 0;
    for part in parts {
        if is_part_accepted(&part, &workflows) {
            res += part.iter().sum::<usize>();
        }
    }

    println!("Total accepted from list: {res}");

    let mut res2 = 0;
    for range in resolve_range(&workflows) {
        let mut total_size = 1;
        for size in range.size {
            total_size *= size
        }
        res2 += total_size;
    }

    println!("Total accepted: {res2}");
}
