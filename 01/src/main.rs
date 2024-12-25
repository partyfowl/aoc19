use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calculate_fuel(mass: i32) -> i32 {
    return mass / 3 - 2;
}

fn calculate_fuel_part_2(mass: i32) -> i32 {
    let mut fuel_mass = calculate_fuel(mass);
    let mut total_fuel_mass = 0;
    while fuel_mass > 0 {
        total_fuel_mass += fuel_mass;
        fuel_mass = calculate_fuel(fuel_mass);
    }
    return total_fuel_mass;
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut part_1_total = 0;
    let mut part_2_total = 0;

    for line in reader.lines() {
        let mass: i32 = line?.parse().unwrap();
        part_1_total += calculate_fuel(mass);
        part_2_total += calculate_fuel_part_2(mass);
    }

    println!("Part 1: {part_1_total}");
    println!("Part 2: {part_2_total}");

    Ok(())
}
