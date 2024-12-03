pub fn part_1(input: &str) -> usize {
    let mut solution = 0;
    for report in input.lines() {
        let report: Vec<u8> = report
            .split_whitespace()
            .map(|line| line.parse::<u8>().unwrap())
            .collect();

        if is_safe_report(&report) {
            solution += 1
        }
    }
    solution
}

fn is_safe_report(report: &[u8]) -> bool {
    let mut decreasing = false;
    if report[0] > report[1] {
        decreasing = true
    };

    for window in report.windows(2) {
        let left = window[0];
        let right = window[1];


        if right == left {
            return false
        }

        if decreasing && right > left {
            return false;
        };
        if !decreasing && left > right {
            return false
        }

        let difference = {
            if decreasing {
                left - right
            } else {
                right - left
            }
        };

        if difference < 1 || difference > 3 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    pub const INPUT: &'static str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-02"));

    #[test]
    fn test_part_01() {
        assert_eq!(part_1(INPUT), 624)
    }
}
