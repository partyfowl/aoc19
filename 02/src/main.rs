use std::fs::read_to_string;
use std::io::Result;

fn intcode(mut instructions: Vec<i32>) -> i32 {
    let mut i: usize = 0;
    while i < instructions.len() && instructions[i] != 99 {
        let opcode = instructions[i];
        let address1: usize = instructions[i + 1].try_into().unwrap();
        let address2: usize = instructions[i + 2].try_into().unwrap();
        let output_address: usize = instructions[i + 3].try_into().unwrap();

        let value1 = instructions[address1];
        let value2 = instructions[address2];

        match opcode {
            1 => instructions[output_address] = value1 + value2,
            2 => instructions[output_address] = value1 * value2,
            _ => break,
        }
        i += 4;
    }
    return instructions[0];
}

fn main() -> Result<()> {
    let instructions: Vec<i32> = read_to_string("input.txt")?
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    let mut part_1_instructions = instructions.clone();
    part_1_instructions[1] = 12;
    part_1_instructions[2] = 2;
    let part_1_answer = intcode(part_1_instructions);

    println!("Part 1: {part_1_answer}");

    let mut output: i32 = 0;
    let expected = 19690720;
    let mut noun = 0;
    let mut verb = 0;

    let mut part_2_answer = 0;

    while noun <= 99 && output != expected {
        while verb <= 99 && output != expected {
            let mut part_2_instructions = instructions.clone();
            part_2_instructions[1] = noun;
            part_2_instructions[2] = verb;
            output = intcode(part_2_instructions);

            if output == expected {
                part_2_answer = 100 * noun + verb;
            }
            verb += 1;
        }
        noun += 1;
        verb = 0;
    }

    println!("Part 2: {part_2_answer}");

    Ok(())
}
