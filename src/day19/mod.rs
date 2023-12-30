use std::{collections::HashMap, ops::RangeInclusive, str::Lines};

use crate::solver::Solver;

const INPUT_WORKFLOW_NAME: &str = "in";

#[derive(Clone, Copy)]
enum ConditionalOperator {
    GreaterThan,
    LesserThan,
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }

    fn field_value(&self, field: Field) -> i64 {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }
}

#[derive(Clone, PartialEq)]
enum RuleOutcome<'a> {
    Accepted,
    Rejected,
    Redirected(&'a str),
    None,
}

#[derive(Clone, Copy)]
enum Field {
    X,
    M,
    A,
    S,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            'x' => Field::X,
            'm' => Field::M,
            'a' => Field::A,
            's' => Field::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
struct Predicate {
    field: Field,
    conditional_operator: ConditionalOperator,
    threshold: i64,
}

struct Rule<'a> {
    predicate: Option<Predicate>,
    success_outcome: RuleOutcome<'a>,
}

impl<'a> Rule<'a> {
    fn satisfy(&self, part: &Part) -> RuleOutcome {
        if self.predicate.is_none() {
            return self.success_outcome.clone();
        }

        let predicate = self.predicate.as_ref().unwrap();
        let field = part.field_value(predicate.field);

        let satisfies = match predicate.conditional_operator {
            ConditionalOperator::GreaterThan => field > predicate.threshold,
            ConditionalOperator::LesserThan => field < predicate.threshold,
        };

        if satisfies {
            self.success_outcome.clone()
        } else {
            RuleOutcome::None
        }
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn run(&self, part: &Part) -> RuleOutcome {
        for rule in &self.rules {
            let outcome = rule.satisfy(part);

            if outcome != RuleOutcome::None {
                return outcome;
            }
        }

        unreachable!()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct PartCombinations {
    x: RangeInclusive<i64>,
    m: RangeInclusive<i64>,
    a: RangeInclusive<i64>,
    s: RangeInclusive<i64>,
}

impl Default for PartCombinations {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

impl PartCombinations {
    fn number_of_combinations(&self) -> i64 {
        (self.x.end() - self.x.start() + 1)
            * (self.m.end() - self.m.start() + 1)
            * (self.a.end() - self.a.start() + 1)
            * (self.s.end() - self.s.start() + 1)
    }

    fn field_value(&self, field: Field) -> &RangeInclusive<i64> {
        match field {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }

    fn field_value_mut(&mut self, field: Field) -> &mut RangeInclusive<i64> {
        match field {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }
    }

    // Returns a combination that satisfies the predicate, and a combination that does
    // not satisfy the predicate. These are therefore mutually exclusive.
    fn partition(&self, predicate: Option<Predicate>) -> (Option<Self>, Option<Self>) {
        if predicate.is_none() {
            return (Some(self.clone()), None);
        }

        let predicate = predicate.unwrap();
        let field = self.field_value(predicate.field);

        let (satisfying_range, not_satisfying_range) = match predicate.conditional_operator {
            ConditionalOperator::GreaterThan => {
                if *field.start() <= predicate.threshold {
                    (
                        (predicate.threshold + 1)..=(*field.end()),
                        (*field.start())..=(predicate.threshold),
                    )
                } else {
                    // 0..=-1 is an empty range.
                    (field.clone(), 0..=-1)
                }
            }
            ConditionalOperator::LesserThan => {
                if *field.end() >= predicate.threshold {
                    (
                        (*field.start())..=(predicate.threshold - 1),
                        (predicate.threshold)..=(*field.end()),
                    )
                } else {
                    // 0..=-1 is an empty range.
                    (field.clone(), 0..=-1)
                }
            }
        };

        let satisfying_combination = if satisfying_range.is_empty() {
            None
        } else {
            let mut new_combination = self.clone();
            *new_combination.field_value_mut(predicate.field) = satisfying_range;

            Some(new_combination)
        };

        let not_satisfying_combination = if not_satisfying_range.is_empty() {
            None
        } else {
            let mut new_combination = self.clone();
            *new_combination.field_value_mut(predicate.field) = not_satisfying_range;

            Some(new_combination)
        };

        (satisfying_combination, not_satisfying_combination)
    }
}

struct WorkflowResolver<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
}

struct CombinationState<'a> {
    part_combinations: PartCombinations,
    workflow_name: &'a str,
}

impl<'a> WorkflowResolver<'a> {
    fn evaluate_all_combinations(&self) -> i64 {
        let mut number_of_combinations = 0;

        let mut workflows_to_visit = vec![CombinationState {
            part_combinations: PartCombinations::default(),
            workflow_name: INPUT_WORKFLOW_NAME,
        }];

        while let Some(CombinationState {
            mut part_combinations,
            workflow_name,
        }) = workflows_to_visit.pop()
        {
            let workflow = self.workflows.get(workflow_name).unwrap();

            for Rule {
                predicate,
                success_outcome,
            } in &workflow.rules
            {
                let (satisfying_combination, not_satisfying_combination) =
                    part_combinations.partition(*predicate);
                match (satisfying_combination, success_outcome) {
                    (Some(part_combinations), RuleOutcome::Accepted) => {
                        number_of_combinations += part_combinations.number_of_combinations()
                    }
                    (Some(part_combinations), RuleOutcome::Redirected(workflow_name)) => {
                        workflows_to_visit.push(CombinationState {
                            part_combinations,
                            workflow_name,
                        })
                    }
                    (Some(_), RuleOutcome::Rejected) | (None, _) => {}
                    (Some(_), RuleOutcome::None) => unreachable!(),
                }

                match not_satisfying_combination {
                    Some(not_satisfying_part_combinations) => {
                        part_combinations = not_satisfying_part_combinations
                    }
                    None => break,
                }
            }
        }

        number_of_combinations
    }

    // Returns the sum of their ratings.
    fn run<Ps>(&self, parts: Ps) -> i64
    where
        Ps: IntoIterator<Item = Part>,
    {
        let mut total = 0;
        for part in parts {
            let mut workflow_name = INPUT_WORKFLOW_NAME;
            loop {
                let workflow = self.workflows.get(workflow_name).unwrap();
                let outcome = workflow.run(&part);
                match outcome {
                    RuleOutcome::Accepted => {
                        total += part.sum();
                        break;
                    }
                    RuleOutcome::Rejected => break,
                    RuleOutcome::Redirected(new_workflow_name) => workflow_name = new_workflow_name,
                    RuleOutcome::None => unreachable!(),
                }
            }
        }

        total
    }

    fn generate_predicate(condition: Option<&str>) -> Option<Predicate> {
        if condition.is_none() {
            return None;
        }

        let condition = condition.unwrap();
        let ((field, threshold), conditional_operator) = if condition.contains('<') {
            (
                condition.split_once('<').unwrap(),
                ConditionalOperator::LesserThan,
            )
        } else if condition.contains('>') {
            (
                condition.split_once('>').unwrap(),
                ConditionalOperator::GreaterThan,
            )
        } else {
            unreachable!()
        };
        let threshold = threshold.parse().unwrap();
        let field = field.chars().next().unwrap().into();

        Some(Predicate {
            field,
            conditional_operator,
            threshold,
        })
    }

    fn new(lines: &mut Lines<'a>) -> Self {
        let mut workflows = HashMap::new();
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }

            let (workflow_name, workflow_rules) = line.split_once('{').unwrap();
            let workflow_rules = workflow_rules.strip_suffix('}').unwrap();

            let mut rules = Vec::new();
            for workflow_rule in workflow_rules.split(',') {
                let (condition, outcome) = match workflow_rule.split_once(':') {
                    Some((predicate, outcome)) => (Some(predicate), outcome),
                    None => (None, workflow_rule),
                };

                let success_outcome = match outcome {
                    "A" => RuleOutcome::Accepted,
                    "R" => RuleOutcome::Rejected,
                    workflow_name => RuleOutcome::Redirected(workflow_name),
                };
                let predicate = Self::generate_predicate(condition);

                rules.push(Rule {
                    predicate,
                    success_outcome,
                });
            }

            let workflow = Workflow { rules };
            workflows.insert(workflow_name, workflow);
        }

        Self { workflows }
    }

    fn generate_parts_list(lines: Lines) -> impl Iterator<Item = Part> + '_ {
        lines.map(|line| {
            let line = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
            let mut field_and_values = line.split(',');
            let x = field_and_values
                .next()
                .unwrap()
                .strip_prefix("x=")
                .unwrap()
                .parse()
                .unwrap();
            let m = field_and_values
                .next()
                .unwrap()
                .strip_prefix("m=")
                .unwrap()
                .parse()
                .unwrap();
            let a = field_and_values
                .next()
                .unwrap()
                .strip_prefix("a=")
                .unwrap()
                .parse()
                .unwrap();
            let s = field_and_values
                .next()
                .unwrap()
                .strip_prefix("s=")
                .unwrap()
                .parse()
                .unwrap();

            Part { x, m, a, s }
        })
    }
}

pub struct Day19Solver {}

impl Solver for Day19Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day19/input.txt").unwrap();
        let mut lines = file.lines();
        let resolver = WorkflowResolver::new(&mut lines);
        let parts = WorkflowResolver::generate_parts_list(lines);
        let sum_of_accepted_parts = resolver.run(parts);
        println!("The sum of all approved parts is {}", sum_of_accepted_parts);
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day19/input.txt").unwrap();
        let resolver = WorkflowResolver::new(&mut file.lines());
        let number_of_combinations = resolver.evaluate_all_combinations();
        println!(
            "The total number of valid combinations is {}",
            number_of_combinations
        );
    }
}
