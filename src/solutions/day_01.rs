use super::Solution;
use std::collections::HashMap;

pub struct Day01 {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}
impl Solution for Day01 {
    const DAY_NUMBER: u8 = 01;

    fn prepare() -> Self {
        let input = Self::input();
        let iter = input.split_whitespace();

        let mut left_list: Vec<u32> = Vec::new();
        for num in iter.clone().step_by(2) {
            let num = num.parse::<u32>().unwrap();
            left_list.push(num);
        }

        let mut right_list: Vec<u32> = Vec::new();
        for num in iter.skip(1).step_by(2) {
            let num = num.parse::<u32>().unwrap();
            right_list.push(num);
        }

        Self {
            left_list,
            right_list,
        }
    }

    fn part_1(&self) -> Option<usize> {
        let mut left_list = self.left_list.clone();
        left_list.sort();

        let mut right_list = self.right_list.clone();
        right_list.sort();

        let mut solution: usize = 0;
        for (left, right) in left_list.into_iter().zip(right_list.into_iter()) {
            if left <= right {
                solution += (right - left) as usize
            } else {
                solution += (left - right) as usize
            }
        }

        Some(solution)
    }

    fn part_2(&self) -> Option<usize> {
        let mut right_list_occurances: HashMap<u32, u32> = HashMap::new();

        for right_num in self.right_list.iter() {
            let count: u32 = right_list_occurances
                .get(right_num)
                .map(|e| e.to_owned())
                .unwrap_or_default();

            right_list_occurances.insert(*right_num, count + 1);
        }

        let mut solution: usize = 0;
        for left_num in self.left_list.iter() {
            solution += (*left_num * right_list_occurances.remove(left_num).unwrap_or(0)) as usize
        }

        Some(solution)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_01() {
        let solution = Day01::prepare();
        assert!(solution.part_1() == Some(3569916))
    }

    #[test]
    fn test_part_02() {
        let solution = Day01::prepare();
        assert!(solution.part_2() == Some(26407426))
    }
}
