use std::borrow::BorrowMut;

const DEFAULT_TIMER: isize = 6;
const DEFAULT_TIMER_NEW_FISH: isize = DEFAULT_TIMER + 2;
const DAYS_TO_SIMULATE: isize = 80;

struct Fish {
    timer: isize
}

impl Copy for Fish {}

impl Clone for Fish {
    fn clone(&self) -> Self {
        return Fish {
            timer: self.timer
        }
    }
}

impl Fish {
    fn decrement(&mut self) -> bool {
        self.timer -= 1;
        if self.timer < 0 {
            self.timer = DEFAULT_TIMER;
            return true;
        }

        return false;
    }
}

struct FishPopulation {
    fish: Vec<Fish>
}

impl FishPopulation {
    fn update(&mut self) {
        let mut current_population: Vec<Fish> = self.fish.clone();
        let mut updated_population: Vec<Fish> = Vec::new();
        for fish in current_population.iter_mut() {
            if fish.decrement() {
                updated_population.push(Fish{timer: DEFAULT_TIMER_NEW_FISH});
            }
            updated_population.push(*fish);
        }
        self.fish = updated_population;
    }

    fn count(&self) -> usize {
        return self.fish.len();
    }
}

fn main() {
    let input_string = include_str!("input.txt");
    println!("initial pop: {}", input_string);
    let initial_population: Vec<Fish> = input_string.split(",")
        .map(|s| isize::from_str_radix(s, 10).unwrap())
        .map(|timer| Fish{timer})
        .collect();

    let mut population = FishPopulation{fish: initial_population};
    let len_initial_population = population.fish.len();

    for _ in 0..DAYS_TO_SIMULATE {
        population.update();
    }

    println!("Starting with {} fish, after {} days they have grown to {} fishes", len_initial_population, DAYS_TO_SIMULATE, population.count());
}

