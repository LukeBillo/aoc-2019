use std::fs;
mod opcodes;

type Opcode = opcodes::Opcode;

fn run_opcode(opcode: Opcode, first_input_position: usize, second_input_position: usize, output_position: usize, current_state: &mut Vec<usize>) {
    let evaluated_value = opcodes::evaluate(opcode, current_state[first_input_position], current_state[second_input_position]);
    current_state[output_position] = evaluated_value;
}

fn execute(program: &mut Vec<usize>, current_opcode_index: usize) -> &Vec<usize> {
    let opcode = program[current_opcode_index];
    let converted_opcode = Opcode::from(opcode);

    if converted_opcode == Opcode::Quit {
        return program;
    }

    let next_opcode_index = current_opcode_index + 4;
    if next_opcode_index > program.len() {
        panic!("Opcode was not Opcode::Quit, and next opcode would've been out of bounds");
    }

    let first_input_position = program[current_opcode_index+1];
    let second_input_position = program[current_opcode_index+2];
    let output_position = program[current_opcode_index+3];

    run_opcode(converted_opcode, first_input_position, second_input_position, output_position, program);
    return execute(program, current_opcode_index+4);
}

pub fn run(input_file: &String) {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let split_inputs = contents.split(',');
    let mut program: Vec<usize> = Vec::new();

    for input in split_inputs {
        let converted = usize::from_str_radix(input, 10).unwrap();
        program.push(converted);
    }

    program[1] = 12;
    program[2] = 2;

    println!();
    println!("Starting execution...");
    execute(&mut program, 0);

    for out in program {
        print!("{} ", out);
    }
    println!();
}