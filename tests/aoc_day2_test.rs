use advent_of_code_2022_rust::{
    aoc_day2::{
        parse_round_inputs, parse_round_requests, solution, solution_part2, Hand::*, Round,
        RoundResult::*,
    },
    utils::read_lines,
};

#[test]
fn parse_correctly_first_lines() {
    let lines = read_lines("./tests/day2.txt");
    let first_10_lines: Vec<String> = lines.into_iter().take(10).collect();

    let rounds_parsed = parse_round_inputs(first_10_lines);

    let expected = vec![
        (Scissor, Rock),
        (Rock, Paper),
        (Scissor, Scissor),
        (Paper, Paper),
        (Scissor, Scissor),
        (Rock, Scissor),
        (Paper, Paper),
        (Scissor, Scissor),
        (Scissor, Scissor),
        (Paper, Rock),
    ];
    assert_eq!(rounds_parsed, expected);
}

#[test]
fn calculate_round_points() {
    let scissor_win = Round::new(Scissor, Win);
    assert_eq!(scissor_win.points(), 9);
    let scissor_tie = Round::new(Scissor, Tie);
    assert_eq!(scissor_tie.points(), 6);
    let scissor_lost = Round::new(Scissor, Lost);
    assert_eq!(scissor_lost.points(), 3);

    let paper_win = Round::new(Paper, Win);
    assert_eq!(paper_win.points(), 8);
    let paper_tie = Round::new(Paper, Tie);
    assert_eq!(paper_tie.points(), 5);
    let paper_lost = Round::new(Paper, Lost);
    assert_eq!(paper_lost.points(), 2);

    let rock_win = Round::new(Rock, Win);
    assert_eq!(rock_win.points(), 7);
    let rock_tie = Round::new(Rock, Tie);
    assert_eq!(rock_tie.points(), 4);
    let rock_lost = Round::new(Rock, Lost);
    assert_eq!(rock_lost.points(), 1);
}

#[test]
fn test_solution() {
    let inputs = vec!["A Y", "B X", "C Z"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let total_points_right_player = solution(inputs);

    assert_eq!(total_points_right_player, 15);
}

#[test]
fn parse_correctly_first_lines_with_round_requested_results() {
    let lines = read_lines("./tests/day2.txt");
    let first_10_lines: Vec<String> = lines.into_iter().take(10).collect();

    let rounds_parsed = parse_round_requests(first_10_lines);

    let expected = vec![
        (Scissor, Lost),
        (Rock, Tie),
        (Scissor, Win),
        (Paper, Tie),
        (Scissor, Win),
        (Rock, Win),
        (Paper, Tie),
        (Scissor, Win),
        (Scissor, Win),
        (Paper, Lost),
    ];
    assert_eq!(rounds_parsed, expected);
}

#[test]
fn create_the_requested_round() {
    assert_eq!(Round::from_request((Paper, Lost)), Round::new(Rock, Lost));
}

#[test]
fn test_solution_part2() {
    let inputs = vec!["A Y", "B X", "C Z"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let total_points_right_player = solution_part2(inputs);

    assert_eq!(total_points_right_player, 12);
}
