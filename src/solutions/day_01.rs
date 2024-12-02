use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    let (mut left_list, mut right_list) = input_into_lists(input);
    left_list.sort();
    right_list.sort();

    let mut solution: usize = 0;
    for (left, right) in left_list.into_iter().zip(right_list.into_iter()) {
        if left <= right {
            solution += (right - left) as usize
        } else {
            solution += (left - right) as usize
        }
    }
    return solution;
}

pub fn part_2(input: &str) -> usize {
    let (left_list, right_list) = input_into_lists(input);

    let mut right_list_occurances: HashMap<u32, u32> = HashMap::new();
    for right_num in right_list.iter() {
        let count: u32 = right_list_occurances
            .get(right_num)
            .map(|e| e.to_owned())
            .unwrap_or_default();
        right_list_occurances.insert(*right_num, count + 1);
    }

    let mut solution: usize = 0;
    for left_num in left_list.iter() {
        solution += (*left_num * right_list_occurances.get(left_num).unwrap_or(&0)) as usize
    }

    return solution;
}

fn input_into_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
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

    return (left_list, right_list);
}

#[cfg(test)]
mod test {
    use super::*;
    pub const INPUT: &'static str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-01"));

    #[test]
    fn test_part_01() {
        assert!(part_1(INPUT) == 3569916)
    }

    #[test]
    fn test_part_02() {
        assert!(part_2(INPUT) == 26407426)
    }
}
