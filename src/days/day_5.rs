use advent_of_code_2024::to_u32;

#[derive(PartialEq, Debug)]
struct Rule {
    candidate: u32,
    before: u32,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let parts = s.split("|").into_iter().map(to_u32).collect::<Vec<u32>>();
        let before = parts[0];
        let after = parts[1];
        Rule {
            candidate: before,
            before: after,
        }
    }
}

struct RulesAndUpdates {
    rules: Vec<Rule>,
    updates: Vec<Vec<u32>>,
}

fn to_update(s: &str) -> Vec<u32> {
    s.split(",").map(to_u32).collect::<Vec<u32>>()
}

fn rules_and_updates(input: &str) -> RulesAndUpdates {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let rules = parts[0].lines().map(Rule::from).collect::<Vec<Rule>>();

    let updates = parts[1].lines().map(to_update).collect::<Vec<Vec<u32>>>();

    RulesAndUpdates { rules, updates }
}

fn process_update_part_a(update: Vec<u32>, rules: &Vec<Rule>) -> Option<u32> {
    for (index, &element) in update.iter().enumerate() {
        let preceding = &update[..index];
        let violates_rule = rules
            .iter()
            .any(|rule| rule.candidate == element && preceding.contains(&rule.before));
        if violates_rule {
            return None;
        }
    }
    let middle_index = update.len() / 2;
    update.get(middle_index).copied()
}

fn process_update_part_b(update: Vec<u32>, rules: &Vec<Rule>) -> u32 {
    for (index, &element) in update.iter().enumerate() {
        let preceding = &update[..index];
        let violates_rule = rules
            .iter()
            .any(|rule| rule.candidate == element && preceding.contains(&rule.before));
        if violates_rule {
            let mut reordered_update = update.clone();
            reordered_update.swap(index, index - 1);
            return process_update_part_b(reordered_update, rules);
        }
    }
    let middle_index = update.len() / 2;
    update.get(middle_index).unwrap().to_owned()
}

pub fn part_a(input: &str) -> u32 {
    let parsed_input = rules_and_updates(input);
    let rules = parsed_input.rules;
    let updates = parsed_input.updates;
    updates
        .into_iter()
        .filter_map(|update| process_update_part_a(update, &rules))
        .sum()
}

pub fn part_b(input: &str) -> u32 {
    let parsed_input = rules_and_updates(input);
    let rules = parsed_input.rules;
    let updates = parsed_input.updates;
    let part_a_sum: u32 = updates
        .iter()
        .filter_map(|update| process_update_part_a(update.to_vec(), &rules))
        .sum();
    let part_b_sum: u32 = updates
        .into_iter()
        .map(|update| process_update_part_b(update, &rules))
        .sum();
    part_b_sum - part_a_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_test_file;

    #[test]
    fn test_rules_and_updates() {
        let input = read_test_file(5);
        let parsed_input = rules_and_updates(&input);
        assert_eq!(
            parsed_input.rules[0],
            Rule {
                candidate: 47,
                before: 53
            }
        );
        assert_eq!(parsed_input.updates[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_process_update_part_a_valid_update() {
        let actual = process_update_part_a(
            vec![75, 47, 61, 53, 29],
            &vec![
                Rule {
                    candidate: 75,
                    before: 47,
                },
                Rule {
                    candidate: 47,
                    before: 53,
                },
                Rule {
                    candidate: 61,
                    before: 29,
                },
            ],
        );
        assert_eq!(actual, Some(61));
    }

    #[test]
    fn test_process_update_part_a_invalid_update() {
        let actual = process_update_part_a(
            vec![75, 47, 61, 53, 29],
            &vec![
                Rule {
                    candidate: 75,
                    before: 47,
                },
                Rule {
                    candidate: 47,
                    before: 53,
                },
                Rule {
                    candidate: 61,
                    before: 29,
                },
                Rule {
                    candidate: 53,
                    before: 61,
                },
            ],
        );
        assert_eq!(actual, None);
    }

    #[test]
    fn test_process_update_part_b() {
        let actual = process_update_part_b(
            vec![47, 75, 97, 61, 53],
            &vec![Rule {
                candidate: 97,
                before: 75,
            }],
        );
        assert_eq!(actual, 75);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(5);
        assert_eq!(part_a(&input), 143);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(5);
        assert_eq!(part_b(&input), 123);
    }
}
