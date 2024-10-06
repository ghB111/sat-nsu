use log::{debug, info, warn};
use std::env;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Problem {
    clauses_count: u64,
    variables_count: u64,
}

struct ProblemBody {
    clauses: Vec<Clause>,
}

#[derive(Clone)]
struct Literal {
    negated: bool,
    idx: u64,
}

#[derive(Clone)]
struct Clause {
    var_idxs: Vec<Literal>,
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

    let result = solve(&problem, &problem_body);
    print_result(&result);
}

// TODO just use standard Result
enum Result {
    Satisfiable { values: Vec<bool> },
    Unsatisfiable,
}

fn print_result(result: &Result) {
    match (result) {
        Result::Unsatisfiable => {
            println!("UNSATISFIABLE")
        }
        Result::Satisfiable { values } => {
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

fn solve(problem: &Problem, body: &ProblemBody) -> Result {
    return solve_rec(problem, body, 0, Vec::new());
}

fn solve_rec(
    problem: &Problem,
    body: &ProblemBody,
    cur_idx: usize,
    cur_values: Vec<bool>,
) -> Result {
    assert_eq!(cur_idx, cur_values.len());
    if problem.variables_count as usize == cur_idx {
        if is_solution(problem, body, &cur_values) {
            return Result::Satisfiable { values: cur_values };
        }
        return Result::Unsatisfiable;
    }

    let mut next_true = cur_values.clone();
    next_true.push(true);
    let next_true_solution = solve_rec(problem, body, cur_idx + 1, next_true);
    match &next_true_solution {
        Result::Satisfiable { .. } => {
            return next_true_solution;
        }
        Result::Unsatisfiable => {}
    }

    let mut next_false = cur_values.clone();
    next_false.push(false);
    let next_false_solution = solve_rec(problem, body, cur_idx + 1, next_false);
    match &next_false_solution {
        Result::Satisfiable { .. } => {
            return next_false_solution;
        }
        Result::Unsatisfiable => {}
    }
    return Result::Unsatisfiable;
}

fn is_solution(problem: &Problem, body: &ProblemBody, values: &Vec<bool>) -> bool {
    body.clauses
        .iter()
        .all(|clause| satisfies(clause, problem, values))
}

fn satisfies(clause: &Clause, problem: &Problem, values: &Vec<bool>) -> bool {
    clause
        .var_idxs
        .iter()
        .any(|literal| values[(literal.idx - 1) as usize] != literal.negated)
}

fn get_problem_body(problem: &Problem, clauses: Vec<&str>) -> ProblemBody {
    assert_eq!(problem.clauses_count as usize, clauses.len());
    let clauses_res = clauses
        .iter()
        .map(|clause| to_clause(problem, clause))
        .collect::<Vec<_>>();
    ProblemBody {
        clauses: clauses_res,
    }
}

fn to_clause(problem: &Problem, clause_str: &str) -> Clause {
    let mut clause: Vec<Literal> = clause_str
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .map(|x| {
            if x < 0 {
                Literal {
                    negated: true,
                    idx: x.abs() as u64,
                }
            } else {
                Literal {
                    negated: false,
                    idx: x as u64,
                }
            }
        })
        .collect();

    // todo ugly code
    assert_eq!(clause.last().unwrap().idx, 0u64);
    clause.pop();
    Clause { var_idxs: clause }
}

fn get_problem_description(as_str: &str) -> Problem {
    let words = as_str.split_whitespace().collect::<Vec<&str>>();
    assert!(words.len() == 4);
    assert!(words[0] == "p");
    assert!(words[1] == "cnf");
    let variables_count: u64 = words[2].parse().expect("Error parsing vars count");
    let clauses_count: u64 = words[3].parse().expect("Error parsing clauses count");
    return Problem {
        clauses_count,
        variables_count,
    };
}

/*
 * TODO have fun
 * https://people.sc.fsu.edu/~jburkardt/data/cnf/cnf.html
 * Some odd facts include:
 * The definition of the next clause normally begins on a new line, but may follow, on the same line, the "0" that
 * marks the end of the previous clause.
 *
 * In some examples of CNF files, the definition of the last clause is not terminated by a final '0';
 *
 * In some examples of CNF files, the rule that the variables are numbered from 1 to N is not followed.
 * The file might declare that there are 10 variables, for instance, but allow them to be numbered 2 through 11.
*/
