use advent_of_code_2022_rust::aoc_day5::{parse_cargo, rotate_matrix_right, CrateMove};

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

    let cargo_moves: Vec<CrateMove> = raw_moves
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
    assert_eq!(cargo_moves, expected);
}
