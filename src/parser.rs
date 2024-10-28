use log::{debug, error, trace};

#[derive(Debug, Clone, PartialEq)]
pub struct Problem {
    /// Amount of variables used in |clauses|.
    pub variables_count: u64,
    /// Variables in clauses are indexed 1 through |variables_count|
    pub clauses: Vec<Clause>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub negated: bool,
    pub idx: u64,
}

#[derive(Debug, Clone, PartialEq)]
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

    let problem_desc = get_problem_description(problem_str)?;

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
        .map(|clause| to_clause(problem_desc, clause))
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

fn to_clause(problem_desc: &ProblemDescription, clause_str: &str) -> Result<Clause, String> {
    fn parse(problem_desc: &ProblemDescription, str: &str) -> Result<Literal, String> {
        let result = str
            .parse::<i64>()
            .map(|x| Literal {
                negated: x < 0,
                idx: x.abs() as u64,
            })
            .map_err(|_| format!("Could not parse literal {}", str))?;

        if result.idx == 0 {
            return Err(format!("Idx 0 is invalid in {}", str));
        }
        if result.idx > problem_desc.variables_count {
            return Err(format!("Idx {} is out of bounce for problem", result.idx));
        }
        Ok(result)
    }

    let maybe_clause: Result<Vec<_>, _> = clause_str
        .split_whitespace()
        .map(|x| parse(problem_desc, x))
        .collect();

    let clause = maybe_clause.map_err(|original_err| {
        format!(
            "Could not parse integer in clause: {}, {}",
            clause_str, original_err
        )
    })?;

    Ok(Clause { var_idxs: clause })
}

fn get_problem_description(problem_desc: &str) -> Result<ProblemDescription, String> {
    let words = problem_desc.split_whitespace().collect::<Vec<&str>>();
    if words.len() != 4 {
        return Err(format!(
            "Problem description contains unexpected amount of words: {}",
            words.len()
        ));
    }
    if words[0] != "p" {
        return Err(format!("Problem does not start with p"));
    }
    if words[1] != "cnf" {
        return Err(format!("Problem does not have 'cnf' as seconds word"));
    }

    let variables_count: u64 = words[2]
        .parse()
        .map_err(|_| "Could not parse variables amount")?;
    let clauses_count: u64 = words[3]
        .parse()
        .map_err(|_| "Could not parse clauses count")?;

    Ok(ProblemDescription {
        clauses_count,
        variables_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_CNF: &str = "p cnf 3 4\n\
                              1 2 3 0\n\
                             -1 2 3 0\n\
                              1 -2 3 0\n\
                              1 2 -3 0";

    const CNF_NO_PROBLEM: &str = "\
                              1 2 3 0\n\
                             -1 2 3 0\n\
                              1 -2 3 0\n\
                              1 2 -3 0";

    const SIMPLE_CNF_COMMENTS: &str = "c this is a comment\n\
                              c THIS is a different comment\n\
                              cp also comment\n\
                              p cnf 3 4\n\
                              1 2 3 0\n\
                             -1 2 3 0\n\
                              1 -2 3 0\n\
                              1 2 -3 0";

    const SIMPLE_CNF_ENDS_NEWLINE: &str = "p cnf 3 4\n\
                              1 2 3 0\n\
                             -1 2 3 0\n\
                              1 -2 3 0\n\
                              1 2 -3 0\n";

    const SIMPLE_CNF_NO_ZERO: &str = "p cnf 3 4\n\
                              1 2 3 0\n\
                             -1 2 3 0\n\
                              1 -2 3 0\n\
                              1 2 -3";

    const EMPTY_CNF: &str = "p cnf 200 0";

    fn parsed_simple_cnf() -> Problem {
        Problem {
            variables_count: 3,
            clauses: vec![
                Clause {
                    var_idxs: vec![
                        Literal {
                            negated: false,
                            idx: 1,
                        },
                        Literal {
                            negated: false,
                            idx: 2,
                        },
                        Literal {
                            negated: false,
                            idx: 3,
                        },
                    ],
                },
                Clause {
                    var_idxs: vec![
                        Literal {
                            negated: true,
                            idx: 1,
                        },
                        Literal {
                            negated: false,
                            idx: 2,
                        },
                        Literal {
                            negated: false,
                            idx: 3,
                        },
                    ],
                },
                Clause {
                    var_idxs: vec![
                        Literal {
                            negated: false,
                            idx: 1,
                        },
                        Literal {
                            negated: true,
                            idx: 2,
                        },
                        Literal {
                            negated: false,
                            idx: 3,
                        },
                    ],
                },
                Clause {
                    var_idxs: vec![
                        Literal {
                            negated: false,
                            idx: 1,
                        },
                        Literal {
                            negated: false,
                            idx: 2,
                        },
                        Literal {
                            negated: true,
                            idx: 3,
                        },
                    ],
                },
            ],
        }
    }

    #[test]
    fn simple_cnf() {
        assert_eq!(parsed_simple_cnf(), parse_cnf(SIMPLE_CNF).unwrap());
        assert_eq!(
            parsed_simple_cnf(),
            parse_cnf(SIMPLE_CNF_ENDS_NEWLINE).unwrap()
        );
    }

    #[test]
    fn empty_cnf() {
        let expected = Problem {
            variables_count: 200,
            clauses: vec![],
        };
        assert_eq!(expected, parse_cnf(EMPTY_CNF).unwrap());
    }

    #[test]
    fn no_zero_in_end() {
        assert_eq!(parsed_simple_cnf(), parse_cnf(SIMPLE_CNF_NO_ZERO).unwrap());
    }

    #[test]
    fn has_comments() {
        assert_eq!(parsed_simple_cnf(), parse_cnf(SIMPLE_CNF_COMMENTS).unwrap());
    }

    #[test]
    fn no_problem_error() {
        let error = parse_cnf(CNF_NO_PROBLEM).unwrap_err();
        assert_eq!(error, "File should contain a line starting with 'p ...'");
    }
}
