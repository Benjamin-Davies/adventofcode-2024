use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut lines = input.lines();

    let mut rules = Vec::new();
    for line in (&mut lines).take_while(|l| !l.is_empty()) {
        let (x, y) = line.split_once("|").unwrap();
        rules.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    let mut print_jobs = Vec::new();
    for line in lines {
        let job = line.split(",").map(|n| n.parse().unwrap()).collect();
        print_jobs.push(job);
    }

    (rules, print_jobs)
}

pub fn part1(input: &str) -> u64 {
    let (rules, print_jobs) = parse_input(input);

    print_jobs
        .iter()
        .filter(|job| is_correct_order(job, &rules))
        .map(|job| middle_number(job))
        .sum()
}

fn is_correct_order(job: &[u64], rules: &[(u64, u64)]) -> bool {
    let mut first_indices = BTreeMap::new();
    let mut last_indices = BTreeMap::new();
    for (i, &page) in job.iter().enumerate() {
        first_indices.entry(page).or_insert(i);
        last_indices.insert(page, i);
    }

    for &(x, y) in rules {
        if let (Some(x_last_i), Some(y_first_i)) = (last_indices.get(&x), first_indices.get(&y)) {
            if x_last_i >= y_first_i {
                return false;
            }
        }
    }
    true
}

fn middle_number(job: &[u64]) -> u64 {
    assert_eq!(job.len() % 2, 1);
    job[job.len() / 2]
}

pub fn part2(input: &str) -> u64 {
    let (rules, print_jobs) = parse_input(input);
    let rules_set = rules.iter().copied().collect::<BTreeSet<(u64, u64)>>();

    print_jobs
        .into_iter()
        .filter(|job| !is_correct_order(job, &rules))
        .map(|job| fix_order(job, &rules_set))
        .map(|job| middle_number(&job))
        .sum()
}

fn fix_order(mut job: Vec<u64>, rules_set: &BTreeSet<(u64, u64)>) -> Vec<u64> {
    job.sort_by(|&a, &b| {
        if a == b {
            Ordering::Equal
        } else if rules_set.contains(&(b, a)) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    job
}
