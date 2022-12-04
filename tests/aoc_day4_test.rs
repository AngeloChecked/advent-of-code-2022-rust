use advent_of_code_2022_rust::{
    aoc_day4::{is_contained, is_overlapped, parse_elf_assigments, solution, solution_part2},
    utils::read_lines,
};

#[test]
fn parse_first_3_elf_assigments() {
    let lines = read_lines("./tests/day4.txt").into_iter().take(3).collect();

    let first_3_assigments = parse_elf_assigments(lines);

    let expected = vec![(28..=47, 45..=47), (32..=97, 98..=98), (59..=92, 91..=93)];
    assert_eq!(first_3_assigments, expected);
}

#[test]
fn assigments_are_contained() {
    assert!(is_contained(1..=2, 1..=2));
    assert!(is_contained(1..=2, 0..=2));
    assert!(is_contained(1..=2, 1..=3));
    assert!(is_contained(1..=1, 1..=2));
    assert!(!is_contained(0..=0, 1..=2));
    assert!(!is_contained(0..=3, 1..=2));
}

#[test]
fn n_assigments_fully_contains_the_other() {
    let raw_assigments = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let assigmants_that_fully_contains_others = solution(raw_assigments);

    assert_eq!(assigmants_that_fully_contains_others, 2);
}

#[test]
fn assigments_overlaps() {
    assert!(is_overlapped(1..=2, 1..=2));
    assert!(is_overlapped(1..=2, 0..=2));
    assert!(is_overlapped(0..=3, 1..=2));
    assert!(is_overlapped(1..=1, 1..=2));
    assert!(is_overlapped(2..=3, 1..=2));
    assert!(is_overlapped(2..=2, 1..=2));
    assert!(!is_overlapped(0..=0, 1..=2));
    assert!(!is_overlapped(3..=3, 1..=2));
}

#[test]
fn n_assigments_overlaps_the_other() {
    let raw_assigments = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let assigmants_that_overlaps_others = solution_part2(raw_assigments);

    assert_eq!(assigmants_that_overlaps_others, 4);
}
