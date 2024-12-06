use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Mul,
    CloseBracket,
    Integer,
    Comma,
}

static TOKEN_SEQUENCE: &[Token] = &[
    Token::Mul,
    Token::Integer,
    Token::Comma,
    Token::Integer,
    Token::CloseBracket,
];

#[derive(Clone, Debug, PartialEq)]
enum Conditional {
    Do,
    Dont,
}

fn extract_mul_instruction_idxs(input: &str) -> Vec<(usize, usize)> {
    let mut sequence_step = 0;
    let mut idx = 0;
    let mut instruction_start = 0;
    let mut instructions = Vec::new();
    while idx < input.len() {
        let expected_token = TOKEN_SEQUENCE[sequence_step].clone();

        if expected_token == Token::Mul {
            instruction_start = idx;
            let is_mul = input
                .get(idx..idx + 4)
                .map_or_else(|| false, |s| s == "mul(");
            if is_mul {
                idx += 4;
                sequence_step += 1;
            } else {
                idx += 1;
            }
        } else if expected_token == Token::Integer {
            let next_non_numeric_index = input[idx..]
                .chars()
                .position(|c| !c.is_numeric())
                .unwrap_or(input.len());
            idx += next_non_numeric_index;
            if next_non_numeric_index > 0 {
                sequence_step += 1;
            } else {
                sequence_step = 0;
            }
        } else if expected_token == Token::Comma {
            let is_comma = matches!(input.get(idx..idx + 1), Some(","));
            if is_comma {
                idx += 1;
                sequence_step += 1;
            } else {
                sequence_step = 0;
            }
        } else if expected_token == Token::CloseBracket {
            let is_close_bracket = matches!(input.get(idx..idx + 1), Some(")"));
            sequence_step = 0;
            if is_close_bracket {
                idx += 1;
                instructions.push((instruction_start, idx));
            }
        }
    }
    instructions
}

fn extract_conditional_idxs(input: &str) -> HashMap<usize, Conditional> {
    input
        .match_indices("don't()")
        .map(|(idx, _)| (idx, Conditional::Dont))
        .chain(
            input
                .match_indices("do()")
                .map(|(idx, _)| (idx, Conditional::Do)),
        )
        .collect()
}

fn get_mul_results(input: &str, instruction_idxs: Vec<(usize, usize)>) -> u32 {
    instruction_idxs
        .into_iter()
        .map(|(start, end)| {
            let instruction = input.get(start + 4..end - 1).unwrap();
            let instruction_parts = instruction.split(',').collect::<Vec<&str>>();
            let a = instruction_parts[0].parse::<u32>().unwrap();
            let b = instruction_parts[1].parse::<u32>().unwrap();
            a * b
        })
        .sum::<u32>()
}

pub fn part_a(input: &str) -> u32 {
    let instruction_idxs = extract_mul_instruction_idxs(input);
    get_mul_results(input, instruction_idxs)
}

pub fn part_b(input: &str) -> u32 {
    let instruction_idxs = extract_mul_instruction_idxs(input);
    let instruction_idx_map = instruction_idxs
        .into_iter()
        .collect::<HashMap<usize, usize>>();
    let conditional_idx_map = extract_conditional_idxs(input);
    let mut execute_instructions = true;

    let mut filtered_instructions = Vec::<(usize, usize)>::new();

    let mut idx = 0;
    while idx < input.len() {
        execute_instructions = conditional_idx_map
            .get(&idx)
            .map_or_else(|| execute_instructions, |c| matches!(c, Conditional::Do));

        if execute_instructions {
            let maybe_instruction = instruction_idx_map.get(&idx).map(|end| (idx, *end));
            if let Some(instruction) = maybe_instruction {
                filtered_instructions.push(instruction);
            }
        }
        idx += 1;
    }

    get_mul_results(input, filtered_instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_test_file;

    #[test]
    fn test_extract_instruction_idxs() {
        let expected = vec![(0, 8), (13, 21)];
        let input = "mul(1,2)BBBBmmul(3,4)";
        let actual = extract_mul_instruction_idxs(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_mul_results() {
        let input = "mul(3,5)";
        let instruction_idxs = vec![(0, 8)];
        let expected = 15;
        let actual = get_mul_results(input, instruction_idxs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_conditional_idxs() {
        let input = "adon't()_mul(5,5),do()?";
        let expected = vec![(1, Conditional::Dont), (18, Conditional::Do)]
            .into_iter()
            .collect::<HashMap<usize, Conditional>>();
        let actual = extract_conditional_idxs(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(3);
        assert_eq!(part_a(&input), 161);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(3);
        assert_eq!(part_b(&input), 48);
    }
}
