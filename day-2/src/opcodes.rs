#[derive(PartialEq)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Quit = 99
}

impl From<usize> for Opcode {
    fn from(i: usize) -> Self {
        match i {
            1  => Opcode::Add,
            2  => Opcode::Multiply,
            99 => Opcode::Quit,
            _  => Opcode::Quit
        }
    }
}

pub fn evaluate(opcode: Opcode, first_input: usize, second_input: usize) -> usize {
    match opcode {
        Opcode::Add => add(first_input, second_input),
        Opcode::Multiply => multiply(first_input, second_input),
        _ => panic!("Unrecognised opcode or Opcode::Quit found")
    }
}

fn add(x: usize, y: usize) -> usize {
    x + y
}

fn multiply(x: usize, y: usize) -> usize {
    x * y
} 
