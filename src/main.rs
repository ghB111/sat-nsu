mod parser;
mod solver;

use parser::*;
use solver::*;

use log::debug;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use strum::VariantNames;

#[derive(Debug, strum::EnumString, strum::VariantNames)]
#[strum(serialize_all = "kebab-case")]
enum SolverMethod {
    Recursion,
    NoRecursion,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "args", about = "Args to the sat solver")]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Method to use when solving
    #[structopt(short, long, default_value="recursion", possible_values = SolverMethod::VARIANTS)]
    method: SolverMethod,
}

fn main() {
    env_logger::init();

    let args = Args::from_args();
    debug!("Args: {:?}", args);

    let input_path = args.input;
    debug!("Running sat for input path {:?}", input_path);

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

    let result = match args.method {
        SolverMethod::Recursion => solve_with_recursion(&problem, &problem_body),
        SolverMethod::NoRecursion => solve_no_recursion(&problem, &problem_body),
    };
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
