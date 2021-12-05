use std::cmp::{max, min};
use std::fs;
use regex::Regex;

const LINE_SEGMENT_PATTERN: &str = r"^(\d{1,3}),(\d{1,3})\s->\s(\d{1,3}),(\d{1,3})$";
const GRID_SIZE: isize = 1000;

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}

struct LineSegment {
    a: Point,
    b: Point
}

impl LineSegment {
    fn is_horizontal(&self) -> bool { self.a.y == self.b.y }
    fn is_vertical(&self) -> bool { self.a.x == self.b.x }
    fn is_diagonal(&self) -> bool {
        if self.is_horizontal_or_vertical() {
            return false;
        }
        
        let slope = (self.b.y - self.a.y) / (self.b.x - self.a.x);

        if slope * slope == 1 {
            // println!("Line Segment {},{} -> {},{} is diagonal!", self.a.x, self.a.y, self.b.x, self.b.y);
            return true;
        }

        return false
    }
    fn is_horizontal_or_vertical(&self) -> bool { self.is_vertical() || self.is_horizontal() }
    fn contains(&self, point: Point) -> bool {
        return if self.is_vertical() {
            point.x == self.a.x
                && point.y >= min::<isize>(self.a.y, self.b.y)
                && point.y <= max::<isize>(self.a.y, self.b.y)
        } else if self.is_horizontal() {
            point.y == self.a.y
                && point.x >= min::<isize>(self.a.x, self.b.x)
                && point.x <= max::<isize>(self.a.x, self.b.x)
        } else if self.is_diagonal() {
            let cross_product = (point.y - self.a.y) * (self.b.x - self.a.x) - (point.x - self.a.x) * (self.b.y - self.a.y);

            if cross_product != 0 {
                return false
            }

            let dot_product = (point.x - self.a.x) * (self.b.x - self.a.x) + (point.y - self.a.y) * (self.b.y - self.a.y);
            if dot_product < 0 {
                return false
            }

            let squared_length_ba = (self.b.x - self.a.x)*(self.b.x - self.a.x) + (self.b.y - self.a.y) * (self.b.y - self.a.y);
            if dot_product > squared_length_ba {
                return false
            }

            return true
        } else {
            false
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/bin/05-hydrothermal-venture/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };

    let mut line_segments: Vec<LineSegment> = Vec::new();

    for line in input_string.lines() {
        let r = Regex::new(LINE_SEGMENT_PATTERN).unwrap();
        for cap in r.captures_iter(line) {
            let x1 = &cap[1].parse::<isize>().unwrap();
            let y1 = &cap[2].parse::<isize>().unwrap();
            let x2 = &cap[3].parse::<isize>().unwrap();
            let y2 = &cap[4].parse::<isize>().unwrap();
            line_segments.push(LineSegment{
                a: Point{ x: *x1, y: *y1},
                b: Point{ x: *x2, y: *y2},
            });
        };
    };

    println!("found {} line segments", line_segments.len());

    let mut points_with_more_than_one_overlap: isize = 0;

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let point = Point {x, y};
            let num_overlapping_line_segments = line_segments.iter()
                // .filter(|ls| ls.is_horizontal_or_vertical())// this is for part 1
                .filter(|ls| ls.contains(point))
                .count();
            if num_overlapping_line_segments > 1 {
                points_with_more_than_one_overlap += 1;
            }
        }
    }

    println!("There are {} points at which 2 or more line segments overlap", points_with_more_than_one_overlap);
}

