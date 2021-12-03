use std::fs;
use regex::Regex;

const COMMAND_REGEX: &str = r"^(?P<command>\w+)\s(?P<amount>\d+)$";

fn main() {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let input = fs::read_to_string("./src/bin/02-dive/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        },
    };

    let lines = input_string.lines();
    for line in lines {
        let r = Regex::new(COMMAND_REGEX).unwrap();
        for cap in r.captures_iter(line) {
            let command = &cap[1];
            let amount = &cap[2].parse::<i32>().unwrap();
            match process_command(command, *amount, aim) {
                Ok((h, v, a)) => {
                    horizontal += h;
                    depth += v;
                    aim +=  a;
                },
                Err(e) => eprintln!("{}", e)
            };
        };
    };
    println!("The final result is x = {}, y = {}. Both multiplied are = {}", horizontal, depth, horizontal * depth);
}

fn process_command(command: &str, amount: i32, aim: i32) -> Result<(i32, i32, i32), String> {
    println!("parsed line: {} {}", command, amount);
    return match command {
        "forward" => Ok((amount, amount * aim, 0)),
        "down" => Ok((0, 0, amount)),
        "up" => Ok((0, 0, -amount)),
        _ => Err(format!("unknown command: {}", command))
    }
}

