mod parser;
mod solver;

use parser::*;
use solver::*;

use log::debug;
use std::env;
use std::fs;

enum SolverMethod {
    Recursion,
    NoRecursion
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let input_path = args.get(1).expect("Expected a path to input cnf file");
    debug!("Running sat for input path {input_path}");

    let cnf_contents = fs::read_to_string(input_path).expect("Error reading file");

    debug!("Length of cnf file: {}", cnf_contents.len());

    let mut lines = cnf_contents.lines();
    let problem_line = lines
        .position(|x| x.starts_with("p "))
        .expect("File should contain a line starting with 'p ...'");

    // todo not elegant to split several times
    let problem_str = cnf_contents.lines().skip(problem_line).next().unwrap();

    let problem = get_problem_description(problem_str);

    let clauses_str = cnf_contents
        .lines()
        .skip(problem_line + 1)
        .collect::<Vec<&str>>();
    let problem_body = get_problem_body(&problem, clauses_str);

    // let result = solve_with_recursion(&problem, &problem_body);
    let result = solve_no_recursion(&problem, &problem_body);
    print_result(&result);
}

fn print_result(result: &Solution) {
    match result {
        Solution::Unsatisfiable => {
            println!("UNSATISFIABLE")
        }
        Solution::Satisfiable { values } => {
            println!("SATISFIABLE");
            print!("v ");
            for (idx, value) in values.iter().enumerate() {
                let print_idx = idx + 1;
                let to_print = if *value {
                    print_idx.to_string()
                } else {
                    format!("-{}", print_idx)
                };
                print!("{to_print} ");
            }
            println!("0");
        }
    }
}
