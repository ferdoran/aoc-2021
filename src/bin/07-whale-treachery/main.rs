
fn main() {
    let input_string = include_str!("input.txt");
    let mut crab_positions: Vec<isize> = input_string.split(",")
        .map(|s| isize::from_str_radix(s, 10).unwrap())
        .collect();

    crab_positions.sort();

    let (position, fuel_cost) = calc_fuel_cost_part1(crab_positions.clone());
    println!("Part 1: fuel cost to move to {}: {}", position, fuel_cost);
    println!("Part 2: fuel cost {}", calc_fuel_cost_part2(crab_positions.clone()));
}

    fn calc_fuel_cost_part1(crab_positions: Vec<isize>) -> (isize, isize) {
    let median: isize = crab_positions[crab_positions.len()/2];
    let mut position_counts = [0; 1938];
    crab_positions.iter().for_each(|pos| {
        position_counts[*pos as usize] += 1;
    });

    let fuel_cost: isize = position_counts.iter().enumerate()
        .map(|(position, count)| {
            let diff: isize = (median - position as isize);
            diff.abs() * count
        })
        .sum();

    return (median, fuel_cost);
}


fn calc_fuel_cost_part2(crab_positions: Vec<isize>) -> isize {
    let mut position_counts = [0; 1938];

    crab_positions.iter().for_each(|pos| {
        position_counts[*pos as usize] += 1;
    });

    let mut fuel_costs_per_position = [0; 1938];
    for position in 0..fuel_costs_per_position.len() {
        fuel_costs_per_position[position] = position_counts.iter().enumerate()
            .map(|(pos2, count)| {
                let diff = (position as isize - pos2 as isize).abs();
                let diff_movement_cost = (diff * (diff+1)) / 2; // (n * (n+1)) / 2
                diff_movement_cost * count
            })
            .sum();
    }

    let fuel_cost = fuel_costs_per_position.iter().min().unwrap();
    return *fuel_cost;
}