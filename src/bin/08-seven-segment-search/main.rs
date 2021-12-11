use std::collections::HashMap;

enum Segment {
    Top,
    Middle,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct SignalPattern {
    patterns: Vec<String>,
    digits: Vec<String>
}

impl SignalPattern {
    fn count_unique_values(&self) -> usize {
        return self.digits.iter()
            .map(|d| d.len())
            .filter(|len| *len == 2 || *len == 3 || *len == 4 || *len == 7)
            .map(|_| 1)
            .sum();
    }

    fn calculate_output(&self) -> usize {
        let bottom_right_top_right = self.patterns.iter().filter(|p| p.len() == 2)
            .next()
            .unwrap();

        let top_pattern = self.patterns.iter().filter(|p| p.len() == 3)
            .next()
            .map(|p| p.chars().filter(|c| !bottom_right_top_right.contains(*c)).collect::<String>())
            .unwrap();

        let top_left_middle = self.patterns.iter().filter(|p| p.len() == 4)
            .next()
            .map(|p| p.chars().filter(|c| !bottom_right_top_right.contains(*c)).collect::<String>())
            .unwrap();

        let bottom_left_bottom = self.patterns.iter().filter(|p| p.len() == 7)
            .next()
            .map(|p| p.chars().filter(|c| !bottom_right_top_right.contains(*c) && !top_pattern.contains(*c) && !top_left_middle.contains(*c)).collect::<String>())
            .unwrap();

        let output_str = self.digits.iter()
            .map(|d| {
                match d.len() {
                    2 => "1",
                    3 => "7",
                    4 => "4",
                    5 => {
                        if bottom_right_top_right.chars().all(|c| d.contains(c)) {
                            "3"
                        } else if bottom_left_bottom.chars().all(|c| d.contains(c)) {
                            "2"
                        } else {
                            "5"
                        }
                    },
                    6 => {
                        if bottom_right_top_right.chars().all(|c| d.contains(c))
                            && top_pattern.chars().all(|c| d.contains(c))
                            && bottom_left_bottom.chars().all(|c| d.contains(c)) {
                            "0"
                        } else if top_left_middle.chars().all(|c| d.contains(c))
                            && top_pattern.chars().all(|c| d.contains(c))
                            && bottom_left_bottom.chars().all(|c| d.contains(c)) {
                            "6"
                        } else {
                            "9"
                        }
                    },
                    7 => "8",
                    _ => "0"
                }
            }).collect::<String>();

        return usize::from_str_radix(output_str.as_str(), 10).unwrap();
    }
}

impl From<Vec<String>> for SignalPattern {
    fn from(split: Vec<String>) -> Self {
        let signals = split[0].split_whitespace().map(String::from).collect::<Vec<String>>();
        let digits = split[1].split_whitespace().map(String::from).collect::<Vec<String>>();

        return SignalPattern { patterns: signals, digits};
    }
}

fn main() {
    let input_string = include_str!("input.txt");
    let signal_patterns: Vec<SignalPattern> = input_string.lines()
        .map(|line| line.split("|"))
        .map(|split| split.map(String::from).collect::<Vec<String>>())
        .map(SignalPattern::from)
        .collect();

    println!("there are {} signal patterns", signal_patterns.len());

    let unique_values = calc_unique_number_outputs_part1(&signal_patterns);
    let total_output = calc_output_part2(&signal_patterns);

    println!("Part 1 unique values: {}", unique_values);
    println!("Part 2 total output: {}", total_output);
}

fn calc_unique_number_outputs_part1(signal_patterns: &Vec<SignalPattern>) -> usize {
    return signal_patterns.iter().map(SignalPattern::count_unique_values).sum();
}

fn calc_output_part2(signal_patterns: &Vec<SignalPattern>) -> usize {
    return signal_patterns.iter().map(SignalPattern::calculate_output).sum()
}