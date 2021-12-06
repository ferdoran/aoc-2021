use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/06-lanternfish/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };
}

