use std::collections::HashSet;

struct Grid {
    heights: [usize; 10000]
}

impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<usize> {
        if x >= 0 && x < 100 && y >= 0 && y < 100 {
            return Some(self.heights[(y*100+x) as usize]);
        }
        return None;
    }

    fn get_low_points(&self) -> HashSet<(isize, isize)> {
        let mut low_points: HashSet<(isize, isize)> = HashSet::new();
        for x in 0..100 {
            for y in 0..100 {
                let cur_pos_val = self.get(x, y).unwrap();
                let adjacent_positions = [(x-1, y), (x+1,y), (x, y-1), (x, y+1)];
                if adjacent_positions.iter()
                    .filter_map(|(ax, ay)| self.get(*ax, *ay))
                    .all(|adjacent_value| cur_pos_val < adjacent_value) {
                    low_points.insert((x, y));
                }
            }
        }

        return low_points;
    }

    fn get_low_points_risk_value(&self) -> usize {
        self.get_low_points().iter()
            .flat_map(|(x, y)| self.get(*x, *y))
            .map(|val| val + 1)
            .sum()
    }

    fn get_basins(&self) -> Vec<HashSet<(isize, isize)>> {
        self.get_low_points()
            .iter()
            .map(|(x, y)| {
                let mut basin_points = HashSet::new();
                self.get_basin_points_at(*x, *y, &mut basin_points).clone()
            })
            .collect()
    }

    fn get_basin_points_at<'a>(&self, x: isize, y: isize, basin_points: &'a mut HashSet<(isize, isize)>) -> &'a mut HashSet<(isize, isize)> {
        if basin_points.insert((x, y)) {
            for (ax, ay) in [(x-1, y), (x+1, y), (x, y-1), (x, y+1)] {
                if let Some(value) = self.get(ax, ay) {
                    if value != 9 {
                        self.get_basin_points_at(ax, ay, basin_points);
                    }
                }
            }
        }

        basin_points
    }

    fn get_product_of_three_largest_basins(&self) -> usize {
        let mut basin_sizes: Vec<usize> = self.get_basins().iter()
            .map(|basin| basin.len())
            .collect();

        basin_sizes.sort();
        return basin_sizes.iter().rev().take(3).product();
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let mut height_map: [usize; 10000] = [0; 10000];
    input_string.lines().enumerate()
        .map(|(line_num, line)| {
            (line_num, line.chars().enumerate())
        })
        .for_each(|(line_num, char_enum)| {
            for (i, char) in char_enum {
                height_map[line_num*100+i] =  char.to_string().parse::<usize>().unwrap();
            }
        });

    let grid = Grid { heights: height_map};
    println!("Part 1: Total risk sum is: {}", grid.get_low_points_risk_value());
    println!("Part 2: Product of size of 3 largest basins: {}", grid.get_product_of_three_largest_basins());
}