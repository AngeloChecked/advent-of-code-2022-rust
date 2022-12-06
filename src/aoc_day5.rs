use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct CrateMove {
    pub r#move: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for CrateMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut without_move: std::str::SplitN<char> = s
            .strip_prefix("move ")
            .map(|s| s.splitn(2, ' '))
            .ok_or(())?;
        let r#move = without_move
            .next()
            .ok_or(())?
            .parse::<usize>()
            .map_err(|_| ())?;
        let mut without_from = without_move
            .next()
            .ok_or(())?
            .strip_prefix("from ")
            .map(|s| s.splitn(2, ' '))
            .ok_or(())?;
        let from = without_from
            .next()
            .ok_or(())?
            .parse::<usize>()
            .map_err(|_| ())?;
        let mut without_to = without_from
            .next()
            .ok_or(())?
            .strip_prefix("to ")
            .map(|s| s.splitn(2, ' '))
            .ok_or(())?;
        let to = without_to
            .next()
            .ok_or(())?
            .parse::<usize>()
            .map_err(|_| ())?;

        Ok(CrateMove { r#move, from, to })
    }
}

pub fn parse_cargo(lines: Vec<String>) -> Vec<Vec<char>> {
    let interesting_lines = lines.into_iter().take_while(|l| !(*l).is_empty());
    let cargo_lines: Vec<String> = interesting_lines
        .clone()
        .take(interesting_lines.count() - 1)
        .collect();

    let mut orizontal_cargo: Vec<Vec<Option<char>>> = Vec::new();
    for cargo_line in cargo_lines {
        let chars: Vec<char> = cargo_line.chars().collect();
        let orizontal_crates: Vec<Option<char>> = chars
            .chunks(4)
            .map(|chunk| {
                String::from_iter(chunk)
                    .replace(['[', ']'], "")
                    .trim()
                    .chars()
                    .next()
            })
            .collect();
        orizontal_cargo.push(orizontal_crates);
    }

    let vertical_cargo_with_options = rotate_matrix_right(orizontal_cargo);
    let cargo: Vec<Vec<char>> = vertical_cargo_with_options
        .into_iter()
        .map(|crates| crates.into_iter().flatten().collect())
        .collect();
    cargo
}

pub fn parse_cargo_and_moves(lines: Vec<String>) -> Option<(Vec<Vec<char>>, Vec<CrateMove>)> {
    let cargo_move_divisor_index = lines.iter().position(|l| l.is_empty())?;
    let (cargo_raw, moves_raw) = lines.split_at(cargo_move_divisor_index);
    let cargo = parse_cargo(Vec::from(cargo_raw));
    let moves: Vec<CrateMove> = moves_raw.iter().flat_map(|s| str::parse(s)).collect();
    Some((cargo, moves))
}

pub fn rotate_matrix_right<A>(matrix: Vec<Vec<A>>) -> Vec<Vec<A>>
where
    A: Clone,
{
    let mut result = Vec::new();
    for row_i in 0..matrix[0].len() {
        let mut new_row = Vec::new();
        for column_i in (0..matrix.len()).rev() {
            new_row.push(matrix[column_i][row_i].clone());
        }
        result.push(new_row);
    }
    result
}

pub fn apply_crate_moves_to_cargo(
    cargo: Vec<Vec<char>>,
    crate_moves: Vec<CrateMove>,
) -> Vec<Vec<char>> {
    crate_moves.into_iter().fold(cargo, |cargo, crate_move| {
        apply_crate_move_to_cargo(cargo, crate_move)
    })
}

fn apply_crate_move_to_cargo(mut cargo: Vec<Vec<char>>, crate_move: CrateMove) -> Vec<Vec<char>> {
    for _ in 0..crate_move.r#move {
        if let Some(crate_to_move) = cargo[crate_move.from - 1].pop() {
            cargo[crate_move.to - 1].push(crate_to_move);
        }
    }
    cargo
}

pub fn solution(lines: Vec<String>) -> String {
    let (cargo, crate_moves) = parse_cargo_and_moves(lines).unwrap();
    let new_cargo = apply_crate_moves_to_cargo(cargo, crate_moves);
    String::from_iter(new_cargo.into_iter().filter_map(|s| s.into_iter().last()))
}

pub fn apply_crate_moves_to_cargo_all_together(
    cargo: Vec<Vec<char>>,
    crate_moves: Vec<CrateMove>,
) -> Vec<Vec<char>> {
    crate_moves.into_iter().fold(cargo, |cargo, crate_move| {
        apply_crate_move_to_cargo_all_together(cargo, crate_move)
    })
}

fn apply_crate_move_to_cargo_all_together(
    mut cargo: Vec<Vec<char>>,
    crate_move: CrateMove,
) -> Vec<Vec<char>> {
    let stack_to_pick: Vec<char> = cargo[crate_move.from - 1].clone();
    let separation_index = stack_to_pick.len() - crate_move.r#move;
    let (new_crate_stack, crates_to_move) = stack_to_pick.split_at(separation_index);
    cargo[crate_move.from - 1] = Vec::from(new_crate_stack);
    cargo[crate_move.to - 1].extend(Vec::from(crates_to_move));
    cargo
}

pub fn solution_part2(lines: Vec<String>) -> String {
    let (cargo, crate_moves) = parse_cargo_and_moves(lines).unwrap();
    let new_cargo = apply_crate_moves_to_cargo_all_together(cargo, crate_moves);
    String::from_iter(new_cargo.into_iter().filter_map(|s| s.into_iter().last()))
}
