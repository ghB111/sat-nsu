#[derive(Debug)]
pub struct Problem {
    pub clauses_count: u64,
    pub variables_count: u64,
}

pub struct ProblemBody {
    pub clauses: Vec<Clause>,
}

#[derive(Clone)]
pub struct Literal {
    pub negated: bool,
    pub idx: u64,
}

#[derive(Clone)]
pub struct Clause {
    pub var_idxs: Vec<Literal>,
}

// TODO just use standard Result
pub enum Result {
    Satisfiable { values: Vec<bool> },
    Unsatisfiable,
}

pub fn solve(problem: &Problem, body: &ProblemBody) -> Result {
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

fn satisfies(clause: &Clause, _problem: &Problem, values: &Vec<bool>) -> bool {
    clause
        .var_idxs
        .iter()
        .any(|literal| values[(literal.idx - 1) as usize] != literal.negated)
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
