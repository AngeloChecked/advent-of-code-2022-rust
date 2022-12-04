use std::collections::HashSet;

use advent_of_code_2022_rust::{
    aoc_day3::{
        item_priority_in_both_departments, parse_elf_groups, parse_items, solution, solution_part2,
    },
    utils::read_lines,
};

#[test]
fn parse_first_items_from_file() {
    let lines = read_lines("./tests/day3.txt").into_iter().take(3).collect();

    let items_in_departments = parse_items(lines);
    let expected = vec![
        (
            HashSet::from(['l', 'r', 'p', 'P', 'B', 'V', 'M', 's', 't', 'c', 'L', 'G']),
            HashSet::from(['d', 'C', 'R', 'h', 'D', 'T', 'w', 'l', 'f']),
        ),
        (
            HashSet::from(['H', 't', 'W', 'J', 'g', 'F', 'j', 'N', 'Z']),
            HashSet::from(['T', 'D', 'd', 'h', 'S', 'b', 'w', 'f', 'j', 'C']),
        ),
        (
            HashSet::from(['W', 'H', 'N', 'Z', 'g', 'q', 'n', 'Q', 'm']),
            HashSet::from(['B', 'V', 'L', 'r', 't', 'p', 'M', 's', 'P', 'q']),
        ),
    ];
    assert_eq!(items_in_departments, expected)
}

#[test]
fn get_priority_of_item_in_both_departments() {
    let departmets_items = (
        HashSet::from(['l', 'r', 'p', 'P', 'B', 'V', 'M', 's', 't', 'c', 'L', 'G']),
        HashSet::from(['d', 'C', 'R', 'h', 'D', 'T', 'w', 'l', 'f']),
    );

    let both_item_priority = item_priority_in_both_departments(departmets_items);

    assert_eq!(both_item_priority, 12)
}

#[test]
fn test_solution() {
    let items = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let sum_of_priority_items = solution(items);

    assert_eq!(sum_of_priority_items, 157);
}

#[test]
fn parse_first_2_groups_from_file() {
    let lines = read_lines("./tests/day3.txt").into_iter().take(6).collect();

    let elf_groups = parse_elf_groups(lines);

    let expected = vec![
        (
            HashSet::from([
                'R', 'C', 'c', 'T', 'G', 'p', 'D', 's', 'l', 'V', 'L', 'w', 'f', 'M', 'r', 'B',
                't', 'P', 'h', 'd',
            ]),
            HashSet::from([
                'N', 't', 'S', 'F', 'b', 'T', 'J', 'W', 'w', 'f', 'j', 'h', 'H', 'd', 'g', 'Z',
                'C', 'D',
            ]),
            HashSet::from([
                'Q', 'p', 'g', 'N', 'n', 's', 'B', 'M', 't', 'P', 'V', 'W', 'm', 'Z', 'r', 'H',
                'L', 'q',
            ]),
        ),
        (
            HashSet::from([
                'p', 'l', 'F', 'N', 'G', 'm', 'H', 'Q', 'C', 'j', 't', 'd', 'z', 'h', 'J', 'n',
                'V', 'c', 'R',
            ]),
            HashSet::from(['h', 'L', 'F', 'G', 'C', 'g', 'W', 'N', 'Z', 'J', 'r']),
            HashSet::from([
                'b', 'B', 'f', 'l', 'D', 'd', 's', 'P', 'M', 'H', 'q', 'z', 'N', 't', 'v', 'w',
            ]),
        ),
    ];

    assert_eq!(elf_groups, expected)
}

#[test]
fn test_solution_part2() {
    let items = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let sum_of_priority_items_in_groups = solution_part2(items);

    assert_eq!(sum_of_priority_items_in_groups, 70);
}
