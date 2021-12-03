use std::fs;
use bitvec::prelude::*;

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

    let columns = init_columns(input_string);
    let (gamma, epsilon) = calc_gamma_and_epsilon_bits(columns);

    println!("Submarine consumes {} power. Gamma rate is {}. Epsilon rate is {}", gamma * epsilon, gamma, epsilon);
}

fn calc_gamma_and_epsilon_bits(columns: [Column; COLUMNS]) -> (u32, u32) {
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

    let gamma: u32 = gamma_bits.load_be::<u32>();
    let epsilon: u32 = epsilon_bits.load_be::<u32>();

    return (gamma, epsilon);
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