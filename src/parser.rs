use crate::solver::*;

pub fn get_problem_body(problem: &Problem, clauses: Vec<&str>) -> ProblemBody {
    assert_eq!(problem.clauses_count as usize, clauses.len());
    let clauses_res = clauses
        .iter()
        .map(|clause| to_clause(problem, clause))
        .collect::<Vec<_>>();
    ProblemBody {
        clauses: clauses_res,
    }
}

fn to_clause(_problem: &Problem, clause_str: &str) -> Clause {
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

pub fn get_problem_description(as_str: &str) -> Problem {
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
