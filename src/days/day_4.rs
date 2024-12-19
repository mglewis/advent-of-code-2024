static XMAS: &str = "XMAS";
static XMAS_REVERSED: &str = "SAMX";

fn transpose_input(input: &str) -> String {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let transposed_input = (0..rows[0].len())
        .map(|i| rows.iter().map(|row| row[i]).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    transposed_input
}

fn reorder_to_diagonal_input(input: &str) -> String {
    let width = input.find('\n').unwrap();
    let mut diagonal_input = String::new();
    let search_str = input.replace("\n", "");

    let start_and_len: Vec<(usize, usize)> = (0..width)
        .map(|i| (i, width - i))
        .chain((1..width).map(|i| {
            let start = (i * width) + (i / width);
            let len = width - (i % width);
            (start, len)
        }))
        .collect();

    for (start_idx, len) in start_and_len {
        let mut idx = start_idx;
        let mut str_seq = String::new();
        while str_seq.len() < len {
            str_seq.push(search_str.chars().nth(idx).unwrap());
            idx += width + 1;
        }
        str_seq.push('\n');
        diagonal_input.push_str(&str_seq);
    }

    diagonal_input
}

fn reverse_lines(input: &str) -> String {
    input
        .lines()
        .map(|line| line.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_matches(input: &str) -> u32 {
    let mut xmas_count = 0;
    for (i, _) in input.char_indices() {
        let candidate = &input[i..(i + XMAS.len()).min(input.len())];
        if candidate == XMAS {
            xmas_count += 1;
        } else if candidate == XMAS_REVERSED {
            xmas_count += 1;
        }
    }
    xmas_count
}

pub fn part_a(input: &str) -> u32 {
    let mut xmas_count = 0;

    // search forwards / backwards
    xmas_count += find_matches(input);

    // search up / down
    let transposed_input = transpose_input(input);
    xmas_count += find_matches(&transposed_input);

    // search diagonally (right / left)
    let diagonal_input = reorder_to_diagonal_input(input);
    xmas_count += find_matches(&diagonal_input);

    // search diagonally (left / right)
    let reversed_input = reverse_lines(input);
    let transposed_diagonal_input = reorder_to_diagonal_input(&reversed_input);
    xmas_count += find_matches(&transposed_diagonal_input);

    xmas_count
}

fn is_m_and_s(left: (usize, usize), right: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    let chars: (char, char) = (grid[left.0][left.1], grid[right.0][right.1]);
    chars == ('M', 'S') || chars == ('S', 'M')
}

pub fn part_b(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut matches = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] == 'A' {
                let top_left = (i - 1, j - 1);
                let bottom_right = (i + 1, j + 1);
                let top_right = (i - 1, j + 1);
                let bottom_left = (i + 1, j - 1);

                let valid_left_to_right = is_m_and_s(top_left, bottom_right, &grid);
                let valid_right_to_left = is_m_and_s(top_right, bottom_left, &grid);

                if valid_left_to_right && valid_right_to_left {
                    matches += 1;
                }
            }
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_test_file;

    #[test]
    fn test_find_matches() {
        let input = r#"
            ---XMAS--
            --SAMX---
            ---------
        "#
        .replace(" ", "");
        let clean_input = input.trim();
        assert_eq!(find_matches(clean_input), 2);
    }

    #[test]
    fn test_transposed_input() {
        let input = r#"
            123
            456
            789
        "#
        .replace(" ", "");
        let clean_input = input.trim();
        let expected = r#"
            147
            258
            369
        "#
        .replace(" ", "");
        let clean_expected = expected.trim();
        let actual = transpose_input(&clean_input);
        assert_eq!(actual, clean_expected);
    }

    #[test]
    fn test_diagonal_input_ordering() {
        let input = r#"
            123
            456
            789
        "#
        .replace(" ", "");
        let clean_input = input.trim();
        let expected = r#"
            159
            26
            3
            48
            7
        "#
        .replace(" ", "");
        let clean_expected = expected.trim();
        let binding = reorder_to_diagonal_input(&clean_input);
        let actual = binding.trim();
        assert_eq!(actual, clean_expected);
    }

    #[test]
    fn test_diagonal_input() {
        let input = r#"
            XS###
            XMA##
            #MAM#
            ##ASX
            ###S#
        "#
        .replace(" ", "");
        let clean_input = input.trim();
        let expected = 3;
        let binding = reorder_to_diagonal_input(&clean_input);
        let actual = binding.trim();
        assert_eq!(expected, find_matches(actual));
    }

    #[test]
    fn test_reversed_diagonal_input() {
        let input = r#"
            ###X
            ##M#
            #A##
            S###
        "#
        .replace(" ", "");
        let clean_input = input.trim();
        let expected = 1;
        let reversed_input = reverse_lines(&clean_input);
        let binding = reorder_to_diagonal_input(&reversed_input);
        let actual = binding.trim();
        assert_eq!(expected, find_matches(actual));
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(4);
        assert_eq!(part_a(&input), 18);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(4);
        assert_eq!(part_b(&input), 9);
    }
}
