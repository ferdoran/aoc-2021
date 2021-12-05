use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::{FromStr, SplitWhitespace};

const BOARD_SIZE: usize = 25;

struct Board {
    numbers: [BoardNumber; BOARD_SIZE],
    is_complete: bool,
    id: usize,
}

impl Board {
    fn mark_number(&mut self, number: u32) {
        self.numbers.iter_mut()
            .filter(|num| num.number == number)
            .for_each(|num| num.marked = true);
    }

    fn has_complete_row(&self) -> bool {
        for i in 0..5 {
            let m1 = self.numbers[i*5].marked;
            let m2 = self.numbers[i*5 + 1].marked;
            let m3 = self.numbers[i*5 + 2].marked;
            let m4 = self.numbers[i*5 + 3].marked;
            let m5 = self.numbers[i*5 + 4].marked;

            if m1 && m2 && m3 && m4 && m5 {
                return true;
            }
        }
        return false;
    }

    fn has_complete_column(&self) -> bool {
        for i in 0..4 {
            let m1 = self.numbers[i].marked;
            let m2 = self.numbers[i + 5].marked;
            let m3 = self.numbers[i + 10].marked;
            let m4 = self.numbers[i + 15].marked;
            let m5 = self.numbers[i + 20].marked;

            if m1 && m2 && m3 && m4 && m5 {
                return true;
            }
        }
        return false;
    }

    fn sum_unmarked(&self) -> u32 {
        self.numbers.iter()
            .filter(|num| num.marked == false)
            .map(|bn| bn.number)
            .sum()
    }

    fn print(&self) {
        println!();
        for i in 0..5 {
            println!("{} {} {} {} {}", self.numbers[i*5].number, self.numbers[i*5 + 1].number, self.numbers[i*5 + 2].number, self.numbers[i*5 + 3].number, self.numbers[i*5 + 4].number)
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        return Board {
            numbers: self.numbers,
            is_complete: false,
            id: self.id
        };
    }
}

struct BoardNumber {
    number: u32,
    marked: bool,
}

impl Clone for BoardNumber {
    fn clone(&self) -> Self {
        return BoardNumber {
            number: self.number,
            marked: self.marked,
        };
    }
}

impl Copy for BoardNumber {}

fn main() {
    let input = fs::read_to_string("./src/bin/04-giant-squid/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };

    let mut split = input_string.split_whitespace();

    let mut bingo_inputs: Vec<u32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut tmp_numbers: [BoardNumber; BOARD_SIZE] = [BoardNumber { number: 0, marked: false }; BOARD_SIZE];
    let mut count = 0;
    for s in split {
        if count == 0 && bingo_inputs.is_empty() {
            bingo_inputs = s.split(",")
                .map(|num| u32::from_str(num).unwrap())
                .collect();
            continue;
        }

        match u32::from_str(s) {
            Ok(num) => {
                tmp_numbers[count] = BoardNumber { number: num, marked: false };
                count += 1;
            }
            Err(e) => println!("failed to parse int {}. Error: {:?}", s, e)
        }

        if count == BOARD_SIZE {
            boards.push(Board { numbers: tmp_numbers, is_complete: false, id: boards.len()+1});
            tmp_numbers = [BoardNumber { number: 0, marked: false }; BOARD_SIZE];
            count = 0;
        }
    }

    println!("got {} boards", boards.len());

    find_first_winning_board(&mut bingo_inputs, &mut boards);
    find_last_winning_board(&mut bingo_inputs, &mut boards);
}

fn find_first_winning_board(bingo_inputs: &Vec<u32>, boards: &mut Vec<Board>) {
    for number in bingo_inputs {
        for (index, board) in boards.iter_mut().enumerate() {
            board.mark_number(*number);
            if board.has_complete_row() || board.has_complete_column() {
                let result = board.sum_unmarked() * number;
                println!("Board {} has won first! Result is: {} and winning number is {}", index + 1, result, number);
                return;
            }
        }
    }
}
fn find_last_winning_board(bingo_inputs: &Vec<u32>, boards: &mut Vec<Board>) {
    let mut last_result = 0;
    // let mut unfinished_boards = Vec::new();
    let mut completed_boards: Vec<usize> = Vec::new();
    for number in bingo_inputs {
        for board in boards.iter_mut().filter(|b| !b.is_complete) {
            board.mark_number(*number);
            if board.has_complete_row() || board.has_complete_column() {
                // println!("number {} completed board: {} with result: {}", number, board.id, number * board.sum_unmarked());
                board.is_complete = true;
                last_result = number * board.sum_unmarked();
                completed_boards.push(board.id);
            }
        }
    }

    println!("The last winning board has result {}", last_result)
}
