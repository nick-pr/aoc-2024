mod solutions;

use solutions::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Must provide a day to run as the first arguement");
        return;
    }

    let solution: SolutionRunner = match args[1].as_ref() {
        "day-01" => SolutionRunner::day(1)
            .set_part_1(day_01::part_1)
            .set_part_2(day_01::part_2),

        "day-02" => SolutionRunner::day(2).set_part_1(day_02::part_1),

        _ => {
            eprintln!("day not found");
            return;
        }
    };
    solution.run();
}

struct SolutionRunner {
    day_number: u8,
    input: String,
    part_1: Option<fn(input: &str) -> usize>,
    part_2: Option<fn(input: &str) -> usize>,
}

impl SolutionRunner {
    fn day(day_number: u8) -> Self {
        Self {
            day_number,
            input: advent_of_code_2024::get_input(&format!("day-{:0>2}", day_number)),
            part_1: None,
            part_2: None,
        }
    }
    fn run(self) {
        println!("-------------------");
        println!("Solution for Day {:0>2}", self.day_number);
        println!("-------------------");
        match self.part_1 {
            Some(solution) => println!("Part 1: {}", solution(&self.input)),
            None => println!("Part 1: Unsolved"),
        };

        match self.part_2 {
            Some(solution) => println!("Part 2: {}", solution(&self.input)),
            None => println!("Part 2: Unsolved"),
        };
    }

    fn set_part_1(mut self, p1_solution: fn(input: &str) -> usize) -> Self {
        self.part_1 = Some(p1_solution);
        self
    }

    fn set_part_2(mut self, p2_solution: fn(input: &str) -> usize) -> Self {
        self.part_2 = Some(p2_solution);
        self
    }
}
