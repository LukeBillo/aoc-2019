use std::fs;

fn get_fuel_for_module(mass: i32) -> i32 {
    println!("{}", mass / 3);
    let fuel_needed =  (mass / 3) - 2;

    if fuel_needed <= 0 {
        return 0;
    }
    else {
        return fuel_needed + get_fuel_for_module(fuel_needed);
    }
}

pub fn run(input_file: &String) {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let masses = contents.split_whitespace();

    let mut total_fuel_needed = 0;

    for mass in masses {
        let converted_mass = mass.parse::<i32>().unwrap();
        total_fuel_needed += get_fuel_for_module(converted_mass);
    }

    println!("{}", total_fuel_needed);
}