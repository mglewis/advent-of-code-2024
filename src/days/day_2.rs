use advent_of_code_2024::to_i64;

fn line_to_i64_vec(line: &str) -> Vec<i64> {
    line.split_whitespace().map(to_i64).collect::<Vec<i64>>()
}

fn parse_lines(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|s| line_to_i64_vec(s))
        .collect::<Vec<Vec<i64>>>()
}

fn line_is_safe(line: &Vec<i64>) -> bool {
    let is_ascending = line[0] < line[1];

    for pair in line.windows(2) {
        let diff = pair[1] - pair[0];
        let abs_diff = diff.abs();

        if is_ascending && diff < 0 {
            return false;
        } else if !is_ascending && diff > 0 {
            return false;
        } else if abs_diff > 3 || abs_diff < 1 {
            return false;
        }
    }
    true
}

fn line_combination_is_safe(line: &Vec<i64>) -> bool {
    let line_combinations_with_one_removed = (0..line.len())
        .map(|i| [&line[..i], &line[i + 1..]].concat())
        .collect::<Vec<Vec<i64>>>();
    let combinations_with_original =
        [&line_combinations_with_one_removed[..], &[line.clone()]].concat();
    combinations_with_original
        .iter()
        .map(line_is_safe)
        .any(|b| b)
}

pub fn part_a(input: &str) -> u32 {
    let lines = parse_lines(input);
    let line_safety = lines
        .into_iter()
        .map(|l| line_is_safe(&l))
        .collect::<Vec<bool>>();
    line_safety.into_iter().filter(|&b| b).count() as u32
}

pub fn part_b(input: &str) -> u32 {
    let lines = parse_lines(input);
    let line_safety = lines
        .into_iter()
        .map(|l| line_combination_is_safe(&l))
        .collect::<Vec<bool>>();
    line_safety.into_iter().filter(|&b| b).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_test_file;

    #[test]
    fn test_line_safety_simple_ascending() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(line_is_safe(&input), true);
    }

    #[test]
    fn test_line_safety_simple_descending() {
        let input = vec![5, 4, 3];
        assert_eq!(line_is_safe(&input), true);
    }

    #[test]
    fn test_line_safety_start_descending() {
        let input = vec![5, 4, 5, 6];
        assert_eq!(line_is_safe(&input), false);
    }

    #[test]
    fn test_line_safety_repeating_value() {
        let input = vec![8, 6, 4, 4, 1];
        assert_eq!(line_is_safe(&input), false);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(2);
        assert_eq!(part_a(&input), 2);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(2);
        assert_eq!(part_b(&input), 4);
    }
}
