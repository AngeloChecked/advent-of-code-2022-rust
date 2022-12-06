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
    let interesting_lines = lines.into_iter().take_while(|l| *l != "");
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
                    .replace("[", "")
                    .replace("]", "")
                    .trim()
                    .chars()
                    .nth(0)
            })
            .collect();
        orizontal_cargo.push(orizontal_crates);
    }

    let vertical_cargo_with_options = rotate_matrix_right(orizontal_cargo);
    let cargo: Vec<Vec<char>> = vertical_cargo_with_options
        .into_iter()
        .map(|crates| crates.into_iter().filter_map(|c| c).collect())
        .collect();
    cargo
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
