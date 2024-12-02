pub fn part_a(input: &str) -> u32 {
    1
}

pub fn part_b(input: &str) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_test_file;

    #[test]
    fn test_find_first_digit() {
        let input = read_test_file(1);
        assert_eq!(part_a(&input), 1);
    }
}
