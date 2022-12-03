use advent_of_code_2022_rust::*;

fn main() {
    println!(
        r"
        day1 = {:?}
        day1 part2 = {:?}
        ",
        aoc_day1::solution(utils::read_lines("tests/day1.txt")),
        aoc_day1::solution_part2(utils::read_lines("tests/day1.txt"))
    )
}
