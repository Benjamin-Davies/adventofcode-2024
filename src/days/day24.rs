use std::{collections::BTreeMap, fmt::Write, fs};

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
    let (states, gates) = parse_input(input);

    eval_outputs("z", &gates, &states)
}

pub fn part2(input: &str) -> String {
    let (states, mut gates) = parse_input(input);

    const SWAPS: &[(&str, &str)] = &[
        // Adder 09
        ("hbs", "kfp"),
        // Adder 18
        ("dhq", "z18"),
        // Adder 22
        ("pdg", "z22"),
        // Adder 27
        ("jcp", "z27"),
    ];

    for &(a, b) in SWAPS {
        let x = gates[a];
        let y = gates[b];
        *gates.get_mut(a).unwrap() = y;
        *gates.get_mut(b).unwrap() = x;
    }

    let x = eval_outputs("x", &gates, &states);
    let y = eval_outputs("y", &gates, &states);
    let z = eval_outputs("z", &gates, &states);
    if x + y != z {
        let diff = (x + y) ^ z;
        let first_bit = diff.trailing_zeros();
        println!("Warning: x + y != z");
        println!("The output first differs at bit {first_bit}");
    }

    fs::write("target/day24.dot", gen_graphviz(states, gates)).unwrap();

    let mut swapped = SWAPS.iter().flat_map(|&(a, b)| [a, b]).collect::<Vec<_>>();
    swapped.sort();
    swapped.join(",")
}

fn eval_outputs<'a>(
    prefix: &str,
    gates: &BTreeMap<&'a str, Gate<'a>>,
    states: &BTreeMap<&'a str, bool>,
) -> u64 {
    let mut states = states.clone();
    let output_gates = gates
        .keys()
        .copied()
        .chain(states.keys().copied().collect::<Vec<_>>())
        .filter(|name| name.starts_with(prefix));
    let output_states = output_gates.map(|name| eval_gate(name, &gates, &mut states));
    output_states
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

fn gen_graphviz(states: BTreeMap<&str, bool>, gates: BTreeMap<&str, Gate>) -> String {
    let mut s = String::new();

    writeln!(s, "digraph {{").unwrap();

    for (&wire, _) in &states {
        writeln!(s, "{wire} [shape=box];").unwrap();
    }
    for (&wire, _) in &gates {
        writeln!(s, "{wire} [shape=box];").unwrap();
    }

    for (i, (&result, &op)) in gates.iter().enumerate() {
        match op {
            Gate::And(lhs, rhs) => {
                writeln!(s, "{i} [label=AND];").unwrap();
                writeln!(s, "{lhs} -> {i};").unwrap();
                writeln!(s, "{rhs} -> {i};").unwrap();
                writeln!(s, "{i} -> {result};").unwrap();
            }
            Gate::Or(lhs, rhs) => {
                writeln!(s, "{i} [label=OR];").unwrap();
                writeln!(s, "{lhs} -> {i};").unwrap();
                writeln!(s, "{rhs} -> {i};").unwrap();
                writeln!(s, "{i} -> {result};").unwrap();
            }
            Gate::Xor(lhs, rhs) => {
                writeln!(s, "{i} [label=XOR];").unwrap();
                writeln!(s, "{lhs} -> {i};").unwrap();
                writeln!(s, "{rhs} -> {i};").unwrap();
                writeln!(s, "{i} -> {result};").unwrap();
            }
        }
    }

    writeln!(s, "}}").unwrap();
    s
}
