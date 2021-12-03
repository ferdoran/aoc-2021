use std::fs;

fn main() {
    let mut increases = 0;
    let input = fs::read_to_string("./src/bin/01-sonar-sweep/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        },
    };

    let lines: Vec<&str> = input_string.lines().collect();

    let mut last_depth = 0;
    for i in 0..lines.len() {
        if i + 2 >= lines.len() {
            break;
        }
        let d1  = lines[i].parse::<i32>().unwrap();
        let d2  = lines[i+1].parse::<i32>().unwrap();
        let d3  = lines[i+2].parse::<i32>().unwrap();
        let depth = d1 + d2 + d3;
        if last_depth > 0 && depth - last_depth > 0 {
            increases += 1;
        }
        last_depth = depth
    };

    println!("{} measurements are larger than the previous ones", increases);
}
