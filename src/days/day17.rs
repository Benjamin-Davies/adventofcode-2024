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
    let (mut state, instructions) = parse_input(input);

    let mut output = Vec::new();
    while state.instruction_pointer < instructions.len() {
        step(&mut state, &instructions, &mut output);
    }

    output
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
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
