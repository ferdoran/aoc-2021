use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

const VALID_SYNTAX_PATTERN: &str = r"^((\().*(\))|(\[).*(\])|(\{).*(\})|(<).*(>))$";
const LINE_WITH_INVALID_CLOSING_PATTERN: &str = r"^((\().*(\))|(\[).*(\])|(\{).*(\})|(<).*(>))(?P<fc>[>\}\]\)])+.*$";

fn main() {
    let input_string = include_str!("input.txt");
    let mut invalids: Vec<&str> = Vec::new();

    input_string.lines().enumerate()
        .for_each(|(li, line)| {
            let mut opens: Vec<&str> = Vec::new();
            for char in line.chars() {
                match char.to_string().as_str() {
                    "(" => opens.push("("),
                    "[" => opens.push("["),
                    "{" => opens.push("{"),
                    "<" => opens.push("<"),
                    ")" => if opens.pop().unwrap() != "(" {
                        invalids.push(")");
                        break;
                    },
                    "]" => if opens.pop().unwrap() != "[" {
                        invalids.push("]");
                        break;
                    },
                    "}" => if opens.pop().unwrap() != "{" {
                        invalids.push("}");
                        break;
                    },
                    ">" => if opens.pop().unwrap() != "<" {
                        invalids.push(">");
                        break;
                    },
                    _ => {}
                };
            }
            println!("After line {} there are {} invalids", li, invalids.len());
        });

    let syntax_error_score: usize = invalids.iter()
        .map(|s| match *s {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => 0
        })
        .sum();

    // lazy_static! {
    //     static ref FAULTY_SYNTAX_REGEX: Regex = Regex::new(LINE_WITH_INVALID_CLOSING_PATTERN).unwrap();
    //     static ref VALID_SYNTAX_REGEX: Regex = Regex::new(VALID_SYNTAX_PATTERN).unwrap();
    // }
    //
    // let syntax_error_score: usize = input_string.lines()
    //     .filter(|line| !VALID_SYNTAX_REGEX.is_match(line))
    //     .filter_map(|line| FAULTY_SYNTAX_REGEX.captures(line))
    //     .flat_map(|captures| captures.name("fc"))
    //     .map(|m| m.as_str())
    //     .map(|s| match s {
    //         ")" => 3,
    //         "]" => 57,
    //         "}" => 1197,
    //         ">" => 25137,
    //         _ => 0
    //     })
    //     .sum();

    println!("Part 1: Syntax Error Score = {}", syntax_error_score);
}