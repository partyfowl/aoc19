use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;
use std::io::Result;

fn get_param_values(
    instructions: Vec<i32>,
    start_position: usize,
    non_target_parameters: usize,
) -> Vec<i32> {
    let mut param_values: Vec<i32> = vec![];
    for i in 1..=non_target_parameters {
        let parameter_mode =
            instructions[start_position] / 10_i32.pow((i + 1).try_into().unwrap()) % 10;
        if parameter_mode == 0 {
            // position mode
            let position: usize = instructions[start_position + i].try_into().unwrap();
            param_values.push(instructions[position])
        } else {
            // immediate mode
            param_values.push(instructions[start_position + i])
        }
    }
    return param_values;
}

fn add(instructions: Vec<i32>, start_position: usize) -> i32 {
    let param_values = get_param_values(instructions, start_position, 2);
    return param_values[0] + param_values[1];
}

fn multiply(instructions: Vec<i32>, start_position: usize) -> i32 {
    let param_values = get_param_values(instructions, start_position, 2);
    return param_values[0] * param_values[1];
}

struct IntCodeMachine {
    instructions: Vec<i32>,
    i: usize,
}

impl IntCodeMachine {
    fn new(instructions: Vec<i32>) -> IntCodeMachine {
        IntCodeMachine {
            instructions: instructions,
            i: 0,
        }
    }

    fn proceed_until_halt(&mut self, mut input: VecDeque<i32>) -> (VecDeque<i32>, i32) {
        let len_instructions = self.instructions.len();
        let mut output: VecDeque<i32> = VecDeque::new();
        while self.i < len_instructions && self.instructions[self.i] != 99 {
            let opcode_param: i32 = self.instructions[self.i];

            let opcode_size: u32 = 2;
            let opcode: i32 = opcode_param % 10_i32.pow(opcode_size);

            let target: usize;

            match opcode {
                1 | 2 | 7 | 8 => target = self.instructions[self.i + 3].try_into().unwrap(),
                3 | 4 => target = self.instructions[self.i + 1].try_into().unwrap(),
                5 | 6 => target = usize::MAX, // value not used, but code will not compile without this
                _ => return (output, 1),
            }

            let interval: usize;

            match opcode {
                1 => {
                    self.instructions[target] = add(self.instructions.clone(), self.i);
                    interval = 4;
                }
                2 => {
                    self.instructions[target] = multiply(self.instructions.clone(), self.i);
                    interval = 4;
                }
                3 => {
                    if input.is_empty() {
                        return (output, 2);
                    }
                    self.instructions[target] = input.pop_front().expect("input deque empty");
                    interval = 2;
                }
                4 => {
                    output.push_back(self.instructions[target]);
                    interval = 2;
                }
                5 => {
                    let param_values = get_param_values(self.instructions.clone(), self.i, 2);
                    if param_values[0] != 0 {
                        self.i = param_values[1].try_into().unwrap();
                        interval = 0;
                    } else {
                        interval = 3;
                    }
                }
                6 => {
                    let param_values = get_param_values(self.instructions.clone(), self.i, 2);
                    if param_values[0] == 0 {
                        self.i = param_values[1].try_into().unwrap();
                        interval = 0;
                    } else {
                        interval = 3;
                    }
                }
                7 => {
                    let param_values = get_param_values(self.instructions.clone(), self.i, 2);
                    self.instructions[target] = if param_values[0] < param_values[1] {
                        1
                    } else {
                        0
                    };
                    interval = 4;
                }
                8 => {
                    let param_values = get_param_values(self.instructions.clone(), self.i, 2);
                    self.instructions[target] = if param_values[0] == param_values[1] {
                        1
                    } else {
                        0
                    };
                    interval = 4;
                }
                _ => return (output, 1),
            }

            self.i += interval;
        }
        return (output, 0);
    }
}

fn parse() -> Result<Vec<i32>> {
    return Ok(fs::read_to_string("input.txt")?
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect());
}

fn main() -> Result<()> {
    let instructions = parse()?;

    let mut part_1_answer = 0;

    for combination in vec![0, 1, 2, 3, 4].iter().permutations(5) {
        let mut signal = 0;
        for a in combination {
            let mut intcode_machine = IntCodeMachine::new(instructions.clone());
            let (output, _) = intcode_machine.proceed_until_halt(VecDeque::from([*a, signal]));
            signal = *output.back().expect("empty output :(");
        }

        if signal > part_1_answer {
            part_1_answer = signal;
        }
    }

    println!("Part 1: {:?}", part_1_answer);

    let mut highest = 0;

    for combination in vec![5, 6, 7, 8, 9].iter().permutations(5) {
        let mut intcode_machines: Vec<IntCodeMachine> = (0..5)
            .map(|_| IntCodeMachine::new(instructions.clone()))
            .collect();

        let mut i = 0;
        let mut output: VecDeque<i32> = VecDeque::new();
        loop {
            if i == 0 {
                (output, _) =
                    intcode_machines[i].proceed_until_halt(VecDeque::from([*combination[i], 0]))
            } else if i < 5 {
                output.push_front(*combination[i]);
                (output, _) = intcode_machines[i].proceed_until_halt(output)
            } else {
                let return_code;
                (output, return_code) = intcode_machines[i % 5].proceed_until_halt(output);
                if (return_code == 0) & (i % 5 == 4) {
                    if let Some(answer) = output.back() {
                        if *answer > highest {
                            highest = *answer;
                        }
                    }
                    break;
                }
            }

            i += 1
        }
    }

    println!("Part 2: {:?}", highest);

    Ok(())
}
