use advent_of_code_2022_rust::aoc_day6::first_4chunk_without_repetition;

#[test]
fn get_all_4chunk_while_encounter_first_without_repetition() {
    let result = first_4chunk_without_repetition("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());

    assert_eq!(
        vec!["zcfz", "cfzf", "fzfw", "zfwz", "fwzz", "wzzq", "zzqf", "zqfr"],
        result
    );
}
