#[derive(Debug, Clone, Copy)]
struct State {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
}

fn parse_input(input: &str) -> (State, Vec<u8>) {
    let mut lines = input.lines();

    let register_a = lines
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let register_b = lines
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let register_c = lines
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();
    assert!(lines.next().unwrap().is_empty());
    let state = State {
        register_a,
        register_b,
        register_c,
        instruction_pointer: 0,
    };

    let instructions = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    (state, instructions)
}

pub fn part1(input: &str) -> String {
    let (state, instructions) = parse_input(input);

    let output = run_program(state, &instructions);

    output
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(input: &str) -> u64 {
    let (state, instructions) = parse_input(input);

    for i in 0..=u16::MAX {
        let start_value = (i as u64) << 48;
        if let Some(value) = search(start_value, 15, state, &instructions) {
            return value;
        }
    }
    unimplemented!()
}

fn search(start_value: u64, i: u32, start_state: State, instructions: &[u8]) -> Option<u64> {
    // The trick here is that, in the program we were given, a given output
    // digit is not affected by earlier input digits. Therefore, a DFS starting
    // at the trailing end of the input should be able to find the solution.

    for j in 0..8 {
        let value = set_octal_digit(start_value, i, j);

        let mut state = start_state;
        state.register_a = value;
        let output = run_program(state, &instructions);

        if output.get(i as usize) == instructions.get(i as usize) {
            if i == 0 {
                return Some(value);
            } else if let Some(v) = search(value, i - 1, start_state, instructions) {
                return Some(v);
            }
        }
    }
    None
}

fn run_program(mut state: State, instructions: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    while state.instruction_pointer < instructions.len() {
        step(&mut state, &instructions, &mut output);
    }
    output
}

fn step(state: &mut State, instructions: &[u8], output: &mut Vec<u8>) {
    let opcode = instructions[state.instruction_pointer];
    let operand = instructions[state.instruction_pointer + 1];
    state.instruction_pointer += 2;

    let literal_operand = operand as u64;
    let combo_operand = match operand {
        0..=3 => literal_operand,
        4 => state.register_a,
        5 => state.register_b,
        6 => state.register_c,
        _ => unimplemented!(),
    };

    match opcode {
        // adv: a = a >> combo
        0 => state.register_a >>= combo_operand,
        // bxl: b = b ^ literal
        1 => state.register_b ^= literal_operand,
        // bst: b = combo % 8
        2 => state.register_b = combo_operand & 7,
        // jnz: if (a != 0) goto literal;
        3 => {
            if state.register_a != 0 {
                state.instruction_pointer = literal_operand as usize;
            }
        }
        // bxc: b = b ^ c
        4 => state.register_b ^= state.register_c,
        // out: print(combo % 8)
        5 => output.push((combo_operand & 7) as u8),
        // bdv: b = a >> combo
        6 => state.register_b = state.register_a >> combo_operand,
        // cdv: c = a >> combo
        7 => state.register_c = state.register_a >> combo_operand,
        _ => unimplemented!(),
    }
}

fn set_octal_digit(mut n: u64, i: u32, d: u8) -> u64 {
    n &= !(7 << (3 * i));
    n |= (d as u64) << (3 * i);
    n
}
