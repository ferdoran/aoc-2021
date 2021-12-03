use std::fs;
use bitvec::prelude::*;
use std::io::Lines;

const COLUMNS: usize = 12;

struct Column {
    zeros: i32,
    ones: i32
}

impl Clone for Column {
    fn clone(&self) -> Self {
        return Column {
            zeros: self.zeros,
            ones: self.ones
        }
    }
}

impl Copy for Column {

}

fn main() {
    let input = fs::read_to_string("./src/bin/03-binary-diagnostic/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        },
    };

    let columns = init_columns(input_string.clone());
    let (gamma, epsilon) = calc_gamma_and_epsilon(columns);

    for i in 0..COLUMNS {
        let col = columns[i];
        let primary_char = if col.ones >= col.zeros { "1" } else { "0" };
        let secondary_char = if col.zeros <= col.ones { "0" } else { "1" };
        println!("Column {}s\tprimary value is {}\t secondary value is {}", i, primary_char, secondary_char);
    }
    println!("Submarine consumes {} power. Gamma rate is {}. Epsilon rate is {}", gamma * epsilon, gamma, epsilon);
    let (oxygen, co2) = calc_oxygen_and_co2(input_string.clone());

    // initially 1097333, 3797, 289
    // later 5815160, 2743, 2120
    println!("Submarine's life support rate is {}. Oxygen generator rating is {}. CO2 scrubber rating is {}", oxygen*co2, oxygen, co2);
}

fn calc_oxygen_and_co2(input: String) -> (u32, u32) {
    // The calculation is still incorrect...
    let mut lines_oxygen: Vec<&str> = input.lines().collect();
    let mut lines_co2: Vec<&str> = lines_oxygen.clone();

    for i in 0..COLUMNS {
        if lines_oxygen.len() == 1 {
            println!("aborting oxygen filtering after {}", i);
            break;
        }
        // don't forget to recalculate the occurrences of 0s and 1s based on the filtered list!
        let col = init_columns_from_given_lines(lines_oxygen.clone())[i];
        let most_common_char = if col.ones >= col.zeros { '1' } else { '0' };

        lines_oxygen = lines_oxygen.iter().filter(|line| {
            match line.chars().nth(i) {
                Some(ch) => ch == most_common_char,
                None => false
            }
        })
            .map(|x| *x)
            .collect();
    }

    for i in 0..COLUMNS {
        if lines_co2.len() == 1 {
            println!("aborting co2 filtering after {}", i);
            break;
        }
        // don't forget to recalculate the occurrences of 0s and 1s based on the filtered list!
        let col = init_columns_from_given_lines(lines_co2.clone())[i];
        let least_common_char = if col.zeros <= col.ones { '0' } else { '1' };

        lines_co2 = lines_co2.iter().filter(|line| {
            match line.chars().nth(i) {
                Some(ch) => ch == least_common_char,
                None => false
            }
        })
            .map(|x| *x)
            .collect();
    }
    println!("oxygen lines: {:?}", lines_oxygen);
    println!("co2 lines: {:?}", lines_co2);

    let mut oxygen_bits: BitVec<Msb0> = BitVec::new();
    let mut co2_bits: BitVec<Msb0> = BitVec::new();
    lines_oxygen.first().unwrap().chars().map(|ch| ch.to_digit(2).unwrap() == 1).for_each(|b| oxygen_bits.push(b));
    lines_co2.first().unwrap().chars().map(|ch| ch.to_digit(2).unwrap() == 1).for_each(|b| co2_bits.push(b));
    let oxygen = oxygen_bits.load::<u32>();
    let co2 = co2_bits.load::<u32>();


    return (oxygen, co2)
}

fn calc_gamma_and_epsilon(columns: [Column; COLUMNS]) -> (u32, u32) {
    let mut gamma_bits: BitVec<Msb0> = BitVec::new();
    let mut epsilon_bits: BitVec<Msb0> = BitVec::new();

    columns.iter()
        .map(|col| {
            if col.zeros > col.ones {
                (false, true)
            } else {
                (true, false)
            }
        })
        .for_each(|(primary, secondary)| {
            gamma_bits.push(primary);
            epsilon_bits.push(secondary);
        });

    let gamma: u32 = gamma_bits.load::<u32>();
    let epsilon: u32 = epsilon_bits.load::<u32>();

    return (gamma, epsilon);
}

fn init_columns_from_given_lines(input_lines: Vec<&str>) -> [Column; COLUMNS] {
    let mut columns: [Column; COLUMNS] = [Column{zeros: 0, ones: 0}; COLUMNS];

    for line in input_lines {
        for (col, char) in line.char_indices() {
            char.to_digit(2)
                .map(|bit| {
                    match bit {
                        0 => columns[col].zeros += 1,
                        1 => columns[col].ones += 1,
                        _ => {}
                    }
                });
        }
    }

    return columns;
}

fn init_columns(input_string: String) -> [Column; COLUMNS] {
    let mut columns: [Column; COLUMNS] = [Column{zeros: 0, ones: 0}; COLUMNS];

    for line in input_string.lines() {
        for (col, char) in line.char_indices() {
            char.to_digit(2)
                .map(|bit| {
                    match bit {
                        0 => columns[col].zeros += 1,
                        1 => columns[col].ones += 1,
                        _ => {}
                    }
                });
        }
    }

    return columns;
}