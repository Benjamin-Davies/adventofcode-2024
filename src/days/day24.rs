use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
enum Gate<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

fn parse_input(input: &str) -> (BTreeMap<&str, bool>, BTreeMap<&str, Gate>) {
    let (states_part, gates_part) = input.split_once("\n\n").unwrap();

    let states = states_part
        .lines()
        .map(|line| {
            let (wire, state_part) = line.split_once(": ").unwrap();
            let state = match state_part {
                "1" => true,
                "0" => false,
                _ => unimplemented!(),
            };
            (wire, state)
        })
        .collect();

    let gates = gates_part
        .lines()
        .map(|line| {
            let (expr_part, result) = line.split_once(" -> ").unwrap();
            let [lhs, op, rhs] = expr_part
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let gate = match op {
                "AND" => Gate::And(lhs, rhs),
                "OR" => Gate::Or(lhs, rhs),
                "XOR" => Gate::Xor(lhs, rhs),
                _ => unimplemented!(),
            };
            (result, gate)
        })
        .collect();

    (states, gates)
}

pub fn part1(input: &str) -> u64 {
    let (mut states, gates) = parse_input(input);

    let z_gates = gates.keys().filter(|name| name.starts_with("z")).copied();
    let z_states = z_gates.map(|name| eval_gate(name, &gates, &mut states));
    z_states
        .enumerate()
        .map(|(i, state)| if state { 1 << i } else { 0 })
        .sum()
}

fn eval_gate<'a>(
    name: &'a str,
    gates: &BTreeMap<&'a str, Gate<'a>>,
    states: &mut BTreeMap<&'a str, bool>,
) -> bool {
    if let Some(&state) = states.get(name) {
        return state;
    }

    let gate = gates[name];
    let state = match gate {
        Gate::And(lhs, rhs) => eval_gate(lhs, gates, states) && eval_gate(rhs, gates, states),
        Gate::Or(lhs, rhs) => eval_gate(lhs, gates, states) || eval_gate(rhs, gates, states),
        Gate::Xor(lhs, rhs) => eval_gate(lhs, gates, states) ^ eval_gate(rhs, gates, states),
    };

    states.insert(name, state);

    state
}
