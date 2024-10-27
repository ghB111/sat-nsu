use log::debug;
use log::trace;

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
pub enum Solution {
    Satisfiable { values: Vec<bool> },
    Unsatisfiable,
}

pub fn solve_with_recursion(problem: &Problem, body: &ProblemBody) -> Solution {
    return solve_rec(problem, body, 0, Vec::new());
}

pub fn solve_no_recursion(problem: &Problem, body: &ProblemBody) -> Solution {
    let mut proposal: Vec<bool> = Vec::new();
    let vec_size = problem.variables_count;
    proposal.resize(vec_size as usize, false);
    loop {
        trace!("Proposal: {:?}", proposal);

        if is_solution(&body, &proposal) {
            return Solution::Satisfiable { values: proposal };
        }

        let has_next = set_next_proposal(&mut proposal).is_ok();
        if !has_next {
            break;
        }
    }
    return Solution::Unsatisfiable;
}

// Modifies prosal to be next proposal. Return Ok for success, Err if given
// proposal was the last possible.
fn set_next_proposal(proposal: &mut Vec<bool>) -> std::result::Result<(), ()> {
    if proposal.is_empty() {
        return Err(());
    }

    let mut maybe_first_zero_idx: Option<usize> = None;
    for (idx, el) in proposal.iter().enumerate() {
        if *el == false {
            maybe_first_zero_idx = Some(idx);
            break;
        }
    }

    debug!("First false idx: {:?}", maybe_first_zero_idx);
    let first_zero_idx = match maybe_first_zero_idx {
        None => return Err(()),
        Some(idx) => idx,
    };

    proposal[first_zero_idx] = true;
    for idx in 0..first_zero_idx {
        proposal[idx] = false;
    }
    Ok(())
}

fn solve_rec(
    problem: &Problem,
    body: &ProblemBody,
    cur_idx: usize,
    cur_values: Vec<bool>,
) -> Solution {
    assert_eq!(cur_idx, cur_values.len());
    if problem.variables_count as usize == cur_idx {
        if is_solution(body, &cur_values) {
            return Solution::Satisfiable { values: cur_values };
        }
        return Solution::Unsatisfiable;
    }

    let mut next_true = cur_values.clone();
    next_true.push(true);
    let next_true_solution = solve_rec(problem, body, cur_idx + 1, next_true);
    match &next_true_solution {
        Solution::Satisfiable { .. } => {
            return next_true_solution;
        }
        Solution::Unsatisfiable => {}
    }

    let mut next_false = cur_values.clone();
    next_false.push(false);
    let next_false_solution = solve_rec(problem, body, cur_idx + 1, next_false);
    match &next_false_solution {
        Solution::Satisfiable { .. } => {
            return next_false_solution;
        }
        Solution::Unsatisfiable => {}
    }
    return Solution::Unsatisfiable;
}

fn is_solution(body: &ProblemBody, values: &Vec<bool>) -> bool {
    body.clauses.iter().all(|clause| satisfies(clause, values))
}

fn satisfies(clause: &Clause, values: &Vec<bool>) -> bool {
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
