mod solutions;

use solutions::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Must provide a day to run as the first arguement");
        return;
    }

    match args[1].as_ref() {
        "day-01" => {
            let solution = Day01::prepare();
            run_solution(solution)
        }
        _ => {
            eprintln!("day not found")
        }
    }
}

fn run_solution<S: Solution>(solution: S) {
    println!("-------------------");
    println!("Solution for Day {:0>2}", S::DAY_NUMBER);
    println!("-------------------");
    match solution.part_1() {
        Some(solution) => println!("Part 1: {}", solution),
        None => println!("Part 1: Unsolved"),
    };

    match solution.part_2() {
        Some(solution) => println!("Part 2: {}", solution),
        None => println!("Part 2: Unsolved"),
    };
}
