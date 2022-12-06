use advent_of_code_2022_rust::aoc_day5::{
    apply_crate_moves_to_cargo, apply_crate_moves_to_cargo_all_together, parse_cargo,
    rotate_matrix_right, solution, solution_part2, CrateMove,
};

#[test]
fn parse_cargo_raw() {
    let lines = vec![
        "[N]     [Q]         [N]            ",
        "[R]     [F] [Q]     [G] [M]        ",
        "[J]     [Z] [T]     [R] [H] [J]    ",
        "[T] [H] [G] [R]     [B] [N] [T]    ",
        "[Z] [J] [J] [G] [F] [Z] [S] [M]    ",
        "[B] [N] [N] [N] [Q] [W] [L] [Q] [S]",
        "[D] [S] [R] [V] [T] [C] [C] [N] [G]",
        "[F] [R] [C] [F] [L] [Q] [F] [D] [P]",
        " 1   2   3   4   5   6   7   8   9 ",
        "",
        "any",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let cargo: Vec<Vec<char>> = parse_cargo(lines);

    let expected = vec![
        vec!['F', 'D', 'B', 'Z', 'T', 'J', 'R', 'N'],
        vec!['R', 'S', 'N', 'J', 'H'],
        vec!['C', 'R', 'N', 'J', 'G', 'Z', 'F', 'Q'],
        vec!['F', 'V', 'N', 'G', 'R', 'T', 'Q'],
        vec!['L', 'T', 'Q', 'F'],
        vec!['Q', 'C', 'W', 'Z', 'B', 'R', 'G', 'N'],
        vec!['F', 'C', 'L', 'S', 'N', 'H', 'M'],
        vec!['D', 'N', 'Q', 'M', 'T', 'J'],
        vec!['P', 'G', 'S'],
    ];
    assert_eq!(cargo, expected);
}

#[test]
fn test_roteate_a_matrix_right() {
    let matrix = vec![
        vec![1, 2, 3, 0],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![0, 2, 3, 4],
    ];

    let matrix = rotate_matrix_right(matrix);

    let expected = vec![
        vec![0, 1, 1, 1, 1],
        vec![2, 2, 2, 2, 2],
        vec![3, 3, 3, 3, 3],
        vec![4, 4, 4, 4, 0],
    ];
    assert_eq!(matrix, expected);
}

#[test]
fn test_parse_cargo_moves() {
    let raw_moves = vec![
        "move 3 from 9 to 4",
        "move 2 from 5 to 2",
        "move 8 from 1 to 9",
    ];

    let crate_moves: Vec<CrateMove> = raw_moves
        .into_iter()
        .map(|s| str::parse(s).unwrap())
        .collect();

    let expected = vec![
        CrateMove {
            r#move: 3,
            from: 9,
            to: 4,
        },
        CrateMove {
            r#move: 2,
            from: 5,
            to: 2,
        },
        CrateMove {
            r#move: 8,
            from: 1,
            to: 9,
        },
    ];
    assert_eq!(crate_moves, expected);
}

#[test]
fn test_pick_crates_one_to_one() {
    let crate_moves = vec![
        CrateMove {
            r#move: 1,
            from: 2,
            to: 1,
        },
        CrateMove {
            r#move: 3,
            from: 1,
            to: 3,
        },
        CrateMove {
            r#move: 2,
            from: 2,
            to: 1,
        },
        CrateMove {
            r#move: 1,
            from: 1,
            to: 2,
        },
    ];
    let cargo = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

    let new_cargo = apply_crate_moves_to_cargo(cargo, crate_moves);

    let expected = vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];
    assert_eq!(new_cargo, expected);
}

#[test]
fn first_of_each_stack() {
    let lines = vec![
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let first_of_each_stack = solution(lines);

    assert_eq!(first_of_each_stack, "CMZ");
}

#[test]
fn test_pick_crates_all_together() {
    let crate_moves = vec![
        CrateMove {
            r#move: 1,
            from: 2,
            to: 1,
        },
        CrateMove {
            r#move: 3,
            from: 1,
            to: 3,
        },
        CrateMove {
            r#move: 2,
            from: 2,
            to: 1,
        },
        CrateMove {
            r#move: 1,
            from: 1,
            to: 2,
        },
    ];
    let cargo = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

    let new_cargo = apply_crate_moves_to_cargo_all_together(cargo, crate_moves);

    let expected = vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']];
    assert_eq!(new_cargo, expected);
}

#[test]
fn first_of_each_stack_solution_part2() {
    let lines = vec![
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let first_of_each_stack = solution_part2(lines);

    assert_eq!(first_of_each_stack, "MCD");
}
