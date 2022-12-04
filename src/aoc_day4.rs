use std::{collections::HashSet, ops::RangeInclusive};

pub fn parse_elf_assigments(
    raw_assigments: Vec<String>,
) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    let mut result = Vec::new();
    for raw_assigment in raw_assigments {
        let (raw_section_one, raw_section_two) = split_two(raw_assigment, ",");
        let (start_one, end_one) = split_two(raw_section_one, "-");
        let (start_two, end_two) = split_two(raw_section_two, "-");
        result.push((
            start_one.parse::<usize>().unwrap()..=end_one.parse::<usize>().unwrap(),
            start_two.parse::<usize>().unwrap()..=end_two.parse::<usize>().unwrap(),
        ))
    }
    result
}

fn split_two(text: String, symbol: &str) -> (String, String) {
    let text_splitted: Vec<&str> = text.split(symbol).collect();
    (
        text_splitted.get(0).unwrap().to_string(),
        text_splitted.get(1).unwrap().to_string(),
    )
}

pub fn is_contained(range_one: RangeInclusive<usize>, range_two: RangeInclusive<usize>) -> bool {
    let first = range_one.clone().into_iter().nth(0).unwrap();
    let last = range_one.into_iter().last().unwrap();

    let first_two = range_two.clone().into_iter().nth(0).unwrap();
    let last_two = range_two.into_iter().last().unwrap();
    first >= first_two && last <= last_two
}

pub fn solution(raw_assigments: Vec<String>) -> usize {
    let assigments = parse_elf_assigments(raw_assigments);
    assigments
        .into_iter()
        .filter(|(one, two)| {
            is_contained(one.clone(), two.clone()) || is_contained(two.clone(), one.clone())
        })
        .count()
}

pub fn is_overlapped(range_one: RangeInclusive<usize>, range_two: RangeInclusive<usize>) -> bool {
    let one: HashSet<usize> = HashSet::from_iter(range_one);
    let two: HashSet<usize> = HashSet::from_iter(range_two);
    one.intersection(&two).count() > 0
}

pub fn solution_part2(raw_assigments: Vec<String>) -> usize {
    let assigments = parse_elf_assigments(raw_assigments);
    assigments
        .into_iter()
        .filter(|(one, two)| is_overlapped(one.clone(), two.clone()))
        .count()
}
