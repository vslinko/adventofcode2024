use std::cmp::Ordering;
use std::collections::HashSet;

fn parse(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let (ordering_rules, updates, _, _) = input.lines().fold(
        (HashSet::new(), vec![], 0, 0),
        |(mut ordering_rules, mut updates, mut file_section, mut line_no), line| {
            line_no += 1;

            if line == "" {
                file_section += 1;
            } else if file_section == 0 {
                let parts = line
                    .split('|')
                    .map(|s| {
                        s.parse()
                            .expect(&format!("Unable to parse number {} at line {}", s, line_no))
                    })
                    .collect::<Vec<i32>>();

                assert_eq!(parts.len(), 2, "Expected two numbers at line {}", line_no);

                ordering_rules.insert((parts[0], parts[1]));
            } else if file_section == 1 {
                let parts = line
                    .split(',')
                    .map(|s| {
                        s.parse()
                            .expect(&format!("Unable to parse number {} at line {}", s, line_no))
                    })
                    .collect::<Vec<i32>>();

                assert!(
                    parts.len() % 2 != 0,
                    "Expected odd number of numbers at line {}",
                    line_no
                );

                updates.push(parts);
            } else {
                panic!("Unexpected third section in file at line {}", line_no);
            }

            (ordering_rules, updates, file_section, line_no)
        },
    );

    (ordering_rules, updates)
}

fn clone_and_sort_by_ordering_rules(
    update: &Vec<i32>,
    ordering_rules: &HashSet<(i32, i32)>,
) -> Vec<i32> {
    let mut ordered_update = update.clone();

    ordered_update.sort_by(|a, b| {
        if ordering_rules.contains(&(*a, *b)) {
            Ordering::Less
        } else if ordering_rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            panic!("No ordering rule for {} and {}", a, b)
        }
    });

    ordered_update
}

pub fn part1(input: &str) -> i32 {
    let (ordering_rules, updates) = parse(input);

    updates.iter().fold(0, |solution1, update| {
        let ordered_update = clone_and_sort_by_ordering_rules(update, &ordering_rules);

        let is_update_ordered = ordered_update
            .iter()
            .zip(update.iter())
            .all(|(a, b)| a == b);

        let ordered_mid_value = ordered_update[ordered_update.len() / 2];

        solution1
            + if is_update_ordered {
                ordered_mid_value
            } else {
                0
            }
    })
}

pub fn part2(input: &str) -> i32 {
    let (ordering_rules, updates) = parse(input);

    updates.iter().fold(0, |solution2, update| {
        let ordered_update = clone_and_sort_by_ordering_rules(update, &ordering_rules);

        let is_update_ordered = ordered_update
            .iter()
            .zip(update.iter())
            .all(|(a, b)| a == b);

        let ordered_mid_value = ordered_update[ordered_update.len() / 2];

        solution2
            + if is_update_ordered {
                0
            } else {
                ordered_mid_value
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day5_part1() {
        let prod_input = read_to_string("./inputs/5.txt").unwrap();
        let prod_output = read_to_string("./outputs/5p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day5_part2() {
        let prod_input = read_to_string("./inputs/5.txt").unwrap();
        let prod_output = read_to_string("./outputs/5p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
