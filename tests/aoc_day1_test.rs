use advent_of_code_2022_rust::aoc_day1::{group_food_calories_by_elf, max_elements};
use advent_of_code_2022_rust::utils::read_lines;

#[test]
fn pick_from_file_correctly_first_food_calories() {
    let lines = read_lines("./tests/day1.txt");

    let first_10_lines: Vec<String> = lines.into_iter().take(10).collect();

    let expected = vec![
        "8890", "13468", "5519", "4939", "", "4580", "3529", "2917", "1329", "4004",
    ];
    assert_eq!(first_10_lines, expected);
}

#[test]
fn cluster_food_calories_list() {
    let calories_list = vec![
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let elf_food_calories_clusters = group_food_calories_by_elf(calories_list);

    let expected_clusters = vec![6000, 4000, 11000, 24000, 10000];
    assert_eq!(elf_food_calories_clusters, expected_clusters);
}

#[test]
fn get_the_3_elfes_with_maximum_of_calories() {
    let elf_calories = vec![6000, 4000, 11000, 24000, 10000];

    let three_elfes_with_max_calories = max_elements(elf_calories, 3);

    let expected_max = vec![10000, 24000, 11000];
    assert_eq!(three_elfes_with_max_calories, expected_max);
}
