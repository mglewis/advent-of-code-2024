use advent_of_code_2024::to_u32;
use itertools::Itertools;

fn parse_line(line: &str) -> (u32, u32) {
    let nums = line.split_whitespace().map(to_u32).collect::<Vec<u32>>();
    (nums[0], nums[1])
}

fn parse_columns(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().map(parse_line).unzip()
}

fn distance(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn similarity_score(a: u32, occurances: u32) -> u32 {
    a * occurances as u32
}

pub fn part_a(input: &str) -> u32 {
    let (col1, col2) = parse_columns(input);
    let sorted_col1 = col1.into_iter().sorted().collect::<Vec<u32>>();
    let sorted_col2 = col2.into_iter().sorted().collect::<Vec<u32>>();
    let distances = sorted_col1
        .iter()
        .zip(sorted_col2.iter())
        .map(|(a, b)| distance(*a, *b));
    return distances.sum();
}

pub fn part_b(input: &str) -> u32 {
    let (col1, col2) = parse_columns(input);
    let col2_counts = col2.into_iter().counts();
    let similarity_scores = col1
        .into_iter()
        .map(|a| similarity_score(a, *col2_counts.get(&a).unwrap_or(&0) as u32));
    return similarity_scores.sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(1);
        assert_eq!(part_a(&input), 11);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(1);
        assert_eq!(part_b(&input), 31);
    }
}
