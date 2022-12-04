use advent_of_code_2022_rust::*;

fn main() {
    println!(
        r"
        day1 = {:?}
        day1 part2 = {:?}
        day2 = {:?}
        day2 part2 = {:?}
        day3 = {:?}
        day3 part2 = {:?}
        day4 = {:?}
        day4 part2 = {:?}
        ",
        aoc_day1::solution(utils::read_lines("tests/day1.txt")),
        aoc_day1::solution_part2(utils::read_lines("tests/day1.txt")),
        aoc_day2::solution(utils::read_lines("tests/day2.txt")),
        aoc_day2::solution_part2(utils::read_lines("tests/day2.txt")),
        aoc_day3::solution(utils::read_lines("tests/day3.txt")),
        aoc_day3::solution_part2(utils::read_lines("tests/day3.txt")),
        aoc_day4::solution(utils::read_lines("tests/day4.txt")),
        aoc_day4::solution_part2(utils::read_lines("tests/day4.txt")),
    )
}
