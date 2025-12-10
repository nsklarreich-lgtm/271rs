use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    data: String,
    next: Vec<String>,
}

#[derive(Debug)]
pub struct Dag {
    nodes: HashMap<String, Node>,
}

impl Dag {
    pub fn new() -> Self {
        Dag { nodes: HashMap::new() }
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.nodes
            .entry(from.clone())
            .or_insert(Node { data: from.clone(), next: vec![] })
            .next
            .push(to.clone());

        self.nodes
            .entry(to.clone())
            .or_insert(Node { data: to, next: vec![] });
    }
}

pub fn read_dag_from_stdin() -> Dag {
    let stdin = io::stdin();
    let mut dag = Dag::new();

    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            let parts: Vec<_> = l.split_whitespace().collect();
            if parts.len() == 2 {
                dag.add_edge(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    dag
}

