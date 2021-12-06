const DAYS_TO_SIMULATE: isize = 256; // Adapt for part 2

fn main() {
    let input_string = include_str!("input.txt");
    let mut timers = [0_usize; 9];

    input_string.split(",")
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .for_each(|t| timers[t] += 1);

    for _ in 0..DAYS_TO_SIMULATE {
        let spawned_fish = timers[0]; // fish that give birth

        for i in 0..timers.len()-1 {
            timers[i] = timers[i+1]; // reduce timers
        }

        timers[6] += spawned_fish; // fish that give birth get their timers reset
        timers[8] = spawned_fish; // add newly born fish
    }

    let result: usize = timers.iter().sum();

    println!("after {} days there are {} fish", DAYS_TO_SIMULATE, result);
}
