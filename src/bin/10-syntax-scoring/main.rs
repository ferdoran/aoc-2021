fn main() {
    let input_string = include_str!("input.txt");
    let mut invalids: Vec<&str> = Vec::new();

    let mut auto_completion_scores: Vec<usize> = input_string.lines().enumerate()
        .filter_map(|(li, line)| {
            let mut opens: Vec<&str> = Vec::new();
            for char in line.chars() {
                match char.to_string().as_str() {
                    "(" => opens.push("("),
                    "[" => opens.push("["),
                    "{" => opens.push("{"),
                    "<" => opens.push("<"),
                    ")" => if opens.pop().unwrap() != "(" {
                        invalids.push(")");
                        return None;
                    },
                    "]" => if opens.pop().unwrap() != "[" {
                        invalids.push("]");
                        return None;
                    },
                    "}" => if opens.pop().unwrap() != "{" {
                        invalids.push("}");
                        return None;
                    },
                    ">" => if opens.pop().unwrap() != "<" {
                        invalids.push(">");
                        return None;
                    },
                    _ => {}
                };
            }
            if opens.len() != 0 {
                Some(opens)
            } else {
                None
            }
        })
        .map(|opens| {
            opens.iter().rev()
                .fold(0, |score, c| {
                    score * 5 + match *c {
                        "(" => 1,
                        "[" => 2,
                        "{" => 3,
                        "<" => 4,
                        _ => 0,
                    }
                })
        })
        .collect();
    auto_completion_scores.sort();

    let syntax_error_score: usize = invalids.iter()
        .map(|s| match *s {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => 0
        })
        .sum();
    let middle_score = auto_completion_scores[auto_completion_scores.len()/2];

    println!("Part 1: Syntax Error Score = {}", syntax_error_score);
    println!("Part 2: Middle Score = {}", middle_score);
}