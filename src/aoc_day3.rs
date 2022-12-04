use std::collections::HashSet;

pub fn parse_items(raw_items: Vec<String>) -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut result = Vec::new();
    for raw_item in raw_items {
        let len = raw_item.len() / 2;
        let items = raw_item.chars().collect::<Vec<char>>();
        let (left_items_department, right_items_departments) = items.split_at(len);
        let left: HashSet<char> = HashSet::from_iter(left_items_department.iter().copied());
        let right: HashSet<char> = HashSet::from_iter(right_items_departments.iter().copied());
        result.push((left, right));
    }
    result
}

pub fn item_priority_in_both_departments(
    (left_dept, right_dept): (HashSet<char>, HashSet<char>),
) -> usize {
    let item_in_both_departhments: HashSet<&char> = left_dept.intersection(&right_dept).collect();
    let alphabet_priority: Vec<char> = (97..123)
        .map(|e| char::from_u32(e).unwrap())
        .chain((65..91).map(|e| char::from_u32(e).unwrap()))
        .collect();
    let char = item_in_both_departhments.iter().last().unwrap();
    println!("{:?}", char);
    alphabet_priority
        .iter()
        .position(|letter| letter == *char)
        .unwrap()
        + 1
}

pub fn solution(raw_items: Vec<String>) -> usize {
    parse_items(raw_items)
        .into_iter()
        .map(item_priority_in_both_departments)
        .sum()
}
