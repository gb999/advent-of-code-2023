use std::{borrow::Borrow, collections::HashMap};

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let mut workflows: HashMap<&str, WorkFlow> = HashMap::new();
    let mut items: Vec<Item> = Vec::new();
    let iter = contents.lines();
    let mut j = 0;
    for (i, line) in iter.enumerate() {
        // parse workflows
        if line == "" {
            j = i;
            break;
        }
        let mut line = line.split("{");
        let label = line.next().unwrap();
        let line = line.next().unwrap();
        let line = &line[0..line.len() - 1];

        let rules: Vec<Box<Rule>> = line.split(",").map(|s| rule_from_string(s)).collect();
        workflows.insert(label, WorkFlow::new(rules));
    }

    for line in contents.lines().skip(j + 1) {
        // parse lines
        let fields: Vec<u64> = line[1..line.len() - 1]
            .split(',')
            .map(|s| (&s[2..]).parse().unwrap())
            .collect();
        items.push(Item {
            x: fields[0],
            m: fields[1],
            a: fields[2],
            s: fields[3],
        })
    }

    let mut sum = 0;

    for item in items {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            match workflow.apply(&item) {
                WorkFlowResult::Accepted => sum += item.get_sum_of_ratings(),
                WorkFlowResult::Rejected => (),
                WorkFlowResult::ContinueAt { label } => {
                    workflow = workflows.get(label.as_str()).unwrap();
                    continue;
                }
            }
            break;
        }
    }
    dbg!(sum);
}

struct WorkFlow {
    rules: Vec<Box<Rule>>,
}

impl WorkFlow {
    fn new(rules: Vec<Box<Rule>>) -> Self {
        WorkFlow { rules }
    }
    fn apply(&self, item: &Item) -> WorkFlowResult {
        for r in &self.rules {
            let f: &Rule = r.borrow();
            if let Some(res) = f(item) {
                return res;
            }
        }
        panic!("Invalid workflow");
    }
}

#[derive(Clone)]
enum WorkFlowResult {
    Accepted,
    Rejected,
    ContinueAt { label: String },
}

#[derive(Debug)]
struct Item {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
impl Item {
    fn get_sum_of_ratings(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

type Rule = dyn Fn(&Item) -> Option<WorkFlowResult>;

fn rule_from_string(s: &str) -> Box<Rule> {
    let mut iter = s.split(':').next().unwrap().chars();
    let category = iter.next().unwrap();
    let relation = iter.next();

    let result = match s.split(':').last().unwrap() {
        "A" => Some(WorkFlowResult::Accepted),
        "R" => Some(WorkFlowResult::Rejected),
        label => Some(WorkFlowResult::ContinueAt {
            label: label.to_string(),
        }),
    };

    if !s.contains(':') {
        return Box::new(move |_| result.clone());
    }
    // if relation contains ':' , there msut be a relation sign and a number
    let relation = relation.unwrap();
    let number: u64 = iter.collect::<String>().parse().unwrap();

    let res = move |item: &Item| {
        let val = match category {
            'x' => item.x,
            'm' => item.m,
            'a' => item.a,
            's' => item.s,
            _ => panic!("Invalid character"),
        };
        match relation {
            '<' => {
                if val < number {
                    return result.clone();
                }
            }
            '>' => {
                if val > number {
                    return result.clone();
                }
            }
            _ => panic!("Invalid input"),
        }
        None
    };
    Box::new(res)
}
