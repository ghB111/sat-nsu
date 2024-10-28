use log::{debug, error, trace};

#[derive(Debug, Clone)]
pub struct Problem {
    /// Amount of variables used in |clauses|.
    pub variables_count: u64,
    /// Variables in clauses are indexed 1 through |variables_count|
    pub clauses: Vec<Clause>,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub negated: bool,
    pub idx: u64,
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub var_idxs: Vec<Literal>,
}

#[derive(Debug)]
struct ProblemDescription {
    pub clauses_count: u64,
    pub variables_count: u64,
}

#[derive(Debug)]
struct ProblemBody {
    pub clauses: Vec<Clause>,
}

pub fn parse_cnf(cnf: &str) -> Result<Problem, String> {
    let mut lines = cnf.lines();

    let problem_str = match lines.find(|x| x.starts_with("p ")) {
        None => return Err("File should contain a line starting with 'p ...'".to_string()),
        Some(x) => x,
    };

    debug!("Problem line: {:?}", problem_str);

    let problem_desc = get_problem_description(problem_str);

    let clauses_str = lines.collect::<Vec<&str>>().join("\n");

    trace!("Clauses text: {:?}", clauses_str);
    let problem_body = get_problem_body(&problem_desc, &clauses_str)?;

    let problem = Problem {
        clauses: problem_body.clauses,
        variables_count: problem_desc.variables_count,
    };
    Ok(problem)
}

fn get_problem_body(
    problem_desc: &ProblemDescription,
    clauses_text: &str,
) -> Result<ProblemBody, String> {
    let mut clauses: Vec<_> = clauses_text
        .split('0')
        .map(|clause| to_clause(clause))
        .collect::<Result<_, _>>()?;

    if clauses
        .last()
        .map(|x| x.var_idxs.is_empty())
        .unwrap_or(false)
    {
        // The last clause is allowed to be empty. In other words,
        // the last clause from text may or may not end with '0'.
        clauses.pop();
    }

    let expected_clauses = problem_desc.clauses_count;
    let actual_clauses = clauses.len() as u64;
    if expected_clauses != actual_clauses {
        error!("Actual clauses: {:?}", clauses);
        return Err(format!(
            "Problem clauses count does \
                not match actual clauses count: {} vs {}",
            expected_clauses, actual_clauses
        ));
    }
    Ok(ProblemBody { clauses })
}

fn to_clause(clause_str: &str) -> Result<Clause, String> {
    let maybe_clause: Result<Vec<_>, _> = clause_str
        .split_whitespace()
        .map(|x| x.parse::<i64>())
        .map(|xx| {
            xx.map(|x| Literal {
                negated: x < 0,
                idx: x.abs() as u64,
            })
        })
        .collect();
    let mut clause =
        maybe_clause.map_err(|_| format!("Could not parse integer in clause: {}", clause_str))?;

    // Remove the last '0'.
    clause.pop();
    Ok(Clause { var_idxs: clause })
}

fn get_problem_description(as_str: &str) -> ProblemDescription {
    let words = as_str.split_whitespace().collect::<Vec<&str>>();
    assert!(words.len() == 4);
    assert!(words[0] == "p");
    assert!(words[1] == "cnf");
    let variables_count: u64 = words[2].parse().expect("Error parsing vars count");
    let clauses_count: u64 = words[3].parse().expect("Error parsing clauses count");
    return ProblemDescription {
        clauses_count,
        variables_count,
    };
}
