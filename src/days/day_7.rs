use advent_of_code_2024::to_i64;
use itertools::Itertools;

struct Line {
    target: i64,
    test_values: Vec<i64>,
}

fn parse_line(line: &str) -> Line {
    let parts = line.split_once(": ").unwrap();
    let target = parts.0.parse::<i64>().unwrap();
    let test_values = parts.1.split(" ").map(to_i64).collect();
    Line {
        target,
        test_values,
    }
}

fn process_line(line: &Line, operators: &Vec<char>) -> bool {
    // Generate all possible operator combinations for the length of test_values - 1
    let operator_combinations = std::iter::repeat(operators)
        .take(line.test_values.len() - 1)
        .multi_cartesian_product();

    for combination in operator_combinations {
        let mut result = line.test_values[0];

        for (i, &op) in combination.iter().enumerate() {
            match op {
                '+' => result += line.test_values[i + 1],
                '*' => result *= line.test_values[i + 1],
                '|' => result = to_i64(&format!("{}{}", result, line.test_values[i + 1])),
                _ => panic!("Invalid operator: {}", op),
            }
        }

        if result == line.target {
            return true;
        }
    }
    false
}

pub fn part_a(input: &str) -> i64 {
    let operators = vec!['+', '*'];
    let lines = input.lines().map(parse_line);
    lines
        .into_iter()
        .filter(|line| process_line(line, &operators))
        .map(|line| line.target)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let operators = vec!['+', '*', '|'];
    let lines = input.lines().map(parse_line);
    lines
        .into_iter()
        .filter(|line| process_line(line, &operators))
        .map(|line| line.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_test_file;

    #[test]
    fn test_parse_line() {
        let input = "5: 1 2 3 4 5";
        let actual = parse_line(input);
        let expected = Line {
            target: 5,
            test_values: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(actual.target, expected.target);
        assert_eq!(actual.test_values, expected.test_values);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(7);
        assert_eq!(part_a(&input), 3749);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(7);
        assert_eq!(part_b(&input), 11387);
    }
}
