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
    let char = item_in_both_departhments.iter().last().unwrap();
    priority_of_char(char)
}

fn priority_of_char(char: &char) -> usize {
    let alphabet_priority: Vec<char> = (97..123)
        .map(|e| char::from_u32(e).unwrap())
        .chain((65..91).map(|e| char::from_u32(e).unwrap()))
        .collect();
    alphabet_priority
        .iter()
        .position(|letter| *letter == *char)
        .unwrap()
        + 1
}

pub fn solution(raw_items: Vec<String>) -> usize {
    parse_items(raw_items)
        .into_iter()
        .map(item_priority_in_both_departments)
        .sum()
}

pub fn parse_elf_groups(
    raw_items: Vec<String>,
) -> Vec<(HashSet<char>, HashSet<char>, HashSet<char>)> {
    let mut result = Vec::new();
    for elf_group_items in raw_items.chunks(3) {
        let groups_as: Vec<HashSet<char>> = elf_group_items
            .iter()
            .map(|g| g.chars().collect::<Vec<char>>())
            .map(HashSet::from_iter)
            .collect();
        let one_group: HashSet<char> = groups_as.get(0).unwrap().clone();
        let two_group: HashSet<char> = groups_as.get(1).unwrap().clone();
        let three_group: HashSet<char> = groups_as.get(2).unwrap().clone();
        result.push((one_group, two_group, three_group));
    }
    result
}

pub fn item_priority_in_groups(
    (group_one, group_two, group_three): (HashSet<char>, HashSet<char>, HashSet<char>),
) -> usize {
    let two_and_three: HashSet<char> = group_two.intersection(&group_three).copied().collect();
    let one_two_and_three: HashSet<&char> = group_one.intersection(&two_and_three).collect();

    let char = one_two_and_three.iter().last().unwrap();
    priority_of_char(char)
}

pub fn solution_part2(raw_items: Vec<String>) -> usize {
    parse_elf_groups(raw_items)
        .into_iter()
        .map(item_priority_in_groups)
        .sum()
}
