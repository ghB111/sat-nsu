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

fn main() -> Result<(), ()> {
    env_logger::init();

    let args = Args::from_args();
    debug!("Args: {:?}", args);

    let input_path = args.input;
    debug!("Running sat for input path {:?}", input_path);

    let cnf_contents = fs::read_to_string(input_path).expect("Error reading file");

    debug!("Length of cnf file: {}", cnf_contents.len());

    let problem = match parse_cnf(&cnf_contents) {
        Ok(problem) => problem,
        Err(parse_error) => {
            println!("Error while parsing cnf: {:?}", parse_error);
            return Err(());
        }
    };

    debug!("Parsed problem: {:?}", problem);

    let result = match args.method {
        SolverMethod::Recursion => solve_with_recursion(&problem),
        SolverMethod::NoRecursion => solve_no_recursion(&problem),
    };

    print_result(&result);
    Ok(())
}

fn print_result(result: &Solution) {
    match result {
        Solution::Unsatisfiable => {
            println!("s UNSATISFIABLE")
        }
        Solution::Satisfiable { values } => {
            println!("s SATISFIABLE");
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
