use std::collections::VecDeque;
use std::fs;
use std::io::Result;

struct IntCodeMachine {
    instructions: Vec<i64>,
    position: usize,
    relative_base: i64,
    len_instructions: usize,
}

impl IntCodeMachine {
    fn new(mut instructions: Vec<i64>) -> IntCodeMachine {
        let len_instructions = instructions.len();
        IntCodeMachine {
            instructions: instructions,
            position: 0,
            relative_base: 0,
            len_instructions: len_instructions,
        }
    }

    fn proceed_until_halt(&mut self, mut input: VecDeque<i64>) -> (VecDeque<i64>, i64) {
        let mut output: VecDeque<i64> = VecDeque::new();
        while self.position < self.len_instructions && self.instructions[self.position] != 99 {
            let opcode_param: i64 = self.instructions[self.position];

            let opcode_size: u32 = 2;
            let opcode: i64 = opcode_param % 10_i64.pow(opcode_size);

            let interval: usize;

            match opcode {
                1 => {
                    let param_values = self.get_param_values(3);
                    let target: usize = param_values[2].0;
                    self.instructions[target] = param_values[0].1 + param_values[1].1;
                    interval = 4;
                }
                2 => {
                    let param_values = self.get_param_values(3);
                    let target: usize = param_values[2].0;
                    self.instructions[target] = param_values[0].1 * param_values[1].1;
                    interval = 4;
                }
                3 => {
                    if input.is_empty() {
                        return (output, 2);
                    }
                    let param_values = self.get_param_values(1);
                    let target: usize = param_values[0].0;
                    self.instructions[target] = input.pop_front().expect("input deque empty");
                    interval = 2;
                }
                4 => {
                    let param_values = self.get_param_values(1);
                    output.push_back(param_values[0].1);
                    interval = 2;
                }
                5 => {
                    let param_values = self.get_param_values(2);
                    if param_values[0].1 != 0 {
                        self.position = param_values[1].1.try_into().unwrap();
                        interval = 0;
                    } else {
                        interval = 3;
                    }
                }
                6 => {
                    let param_values = self.get_param_values(2);
                    if param_values[0].1 == 0 {
                        self.position = param_values[1].1.try_into().unwrap();
                        interval = 0;
                    } else {
                        interval = 3;
                    }
                }
                7 => {
                    let param_values = self.get_param_values(3);
                    let target: usize = param_values[2].0;
                    self.instructions[target] = if param_values[0].1 < param_values[1].1 {
                        1
                    } else {
                        0
                    };
                    interval = 4;
                }
                8 => {
                    let param_values = self.get_param_values(3);
                    let target: usize = param_values[2].0;
                    self.instructions[target] = if param_values[0].1 == param_values[1].1 {
                        1
                    } else {
                        0
                    };
                    interval = 4;
                }
                9 => {
                    let param_values = self.get_param_values(1);
                    let relative_base_change = param_values[0].1;
                    self.relative_base += relative_base_change;
                    interval = 2;
                }

                _ => return (output, 1),
            }

            self.position += interval;
        }
        return (output, 0);
    }

    fn extend_instructions(&mut self, length: usize) {
        for _ in self.instructions.len()..=length {
            self.instructions.push(0);
        }
    }

    fn get_param_values(&mut self, num_parameters: usize) -> Vec<(usize, i64)> {
        let mut param_values: Vec<(usize, i64)> = vec![];
        for i in 1..=num_parameters {
            let parameter_mode =
                self.instructions[self.position] / 10_i64.pow((i + 1).try_into().unwrap()) % 10;
            if parameter_mode == 0 {
                // position mode
                let position: usize = self.instructions[self.position + i].try_into().unwrap();
                self.extend_instructions(position);
                param_values.push((position, self.instructions[position]))
            } else if parameter_mode == 1 {
                // immediate mode
                param_values.push((usize::MAX, self.instructions[self.position + i]))
            } else {
                // relative mode
                let offset = self.instructions[self.position + i];
                let position: usize = (self.relative_base + offset).try_into().unwrap();
                self.extend_instructions(position);
                param_values.push((position, self.instructions[position]))
            }
        }
        return param_values;
    }
}

fn parse() -> Result<Vec<i64>> {
    return Ok(fs::read_to_string("input.txt")?
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect());
}

fn main() -> Result<()> {
    let instructions = parse()?;

    let mut intcode_machine = IntCodeMachine::new(instructions.clone());

    let (tiles, _) = intcode_machine.proceed_until_halt(VecDeque::from([]));

    let mut part_1_answer = 0;

    let mut i = 0;

    while i < tiles.len() {
        if tiles[i + 2] == 2 {
            part_1_answer += 1
        }
        i += 3
    }

    println!("Part 1: {:?}", part_1_answer);

    Ok(())
}
