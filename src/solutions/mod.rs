mod day_01;

pub use day_01::Day01;

pub trait Solution {
    const DAY_NUMBER: u8;

    fn prepare() -> Self;

    fn part_1(&self) -> Option<usize> {
        return None;
    }

    fn part_2(&self) -> Option<usize> {
        return None;
    }

    fn input() -> String {
        let input_path = format!("input/day-{:0>2}", Self::DAY_NUMBER);
        match std::fs::read_to_string(input_path) {
            Ok(input) => input,
            Err(error) => {
                panic!(
                    "Failed to read input for day {}: {}",
                    Self::DAY_NUMBER,
                    error
                )
            }
        }
    }
}
