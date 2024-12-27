use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;
use std::io::{Error, ErrorKind, Result};

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

fn intcode(mut instructions: Vec<i32>, mut input: VecDeque<i32>) -> Result<Vec<i32>> {
    let mut i: usize = 0;
    let mut output: Vec<i32> = vec![];

    let len_instructions = instructions.len();

    while i < len_instructions && instructions[i] != 99 {
        let opcode_param: i32 = instructions[i];

        let opcode_size: u32 = 2;
        let opcode: i32 = opcode_param % 10_i32.pow(opcode_size);

        let target: usize;

        match opcode {
            1 | 2 | 7 | 8 => target = instructions[i + 3].try_into().unwrap(),
            3 | 4 => target = instructions[i + 1].try_into().unwrap(),
            5 | 6 => target = usize::MAX, // value not used, but code will not compile without this
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid opcode: {}", opcode),
                ))
            }
        }

        let interval: usize;

        match opcode {
            1 => {
                instructions[target] = add(instructions.clone(), i);
                interval = 4;
            }
            2 => {
                instructions[target] = multiply(instructions.clone(), i);
                interval = 4;
            }
            3 => {
                instructions[target] = input.pop_front().expect("input deque empty");
                interval = 2;
            }
            4 => {
                output.push(instructions[target]);
                interval = 2;
            }
            5 => {
                let param_values = get_param_values(instructions.clone(), i, 2);
                if param_values[0] != 0 {
                    i = param_values[1].try_into().unwrap();
                    interval = 0;
                } else {
                    interval = 3;
                }
            }
            6 => {
                let param_values = get_param_values(instructions.clone(), i, 2);
                if param_values[0] == 0 {
                    i = param_values[1].try_into().unwrap();
                    interval = 0;
                } else {
                    interval = 3;
                }
            }
            7 => {
                let param_values = get_param_values(instructions.clone(), i, 2);
                instructions[target] = if param_values[0] < param_values[1] {
                    1
                } else {
                    0
                };
                interval = 4;
            }
            8 => {
                let param_values = get_param_values(instructions.clone(), i, 2);
                instructions[target] = if param_values[0] == param_values[1] {
                    1
                } else {
                    0
                };
                interval = 4;
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid opcode: {}", opcode),
                ))
            }
        }

        i += interval;
    }

    return Ok(output);
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
            let input: VecDeque<i32> = VecDeque::from([*a, signal]);
            let output = intcode(instructions.clone(), input)?;
            signal = *output.last().expect("output empty");
        }

        if signal > part_1_answer {
            part_1_answer = signal;
        }
    }
    println!("Part 1: {:?}", part_1_answer);

    Ok(())
}
