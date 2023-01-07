use std::collections::HashSet;

pub fn first_4chunk_without_repetition2(text: String) -> Vec<String> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let mut chars = text.chars();

    let mut chunk4 = Vec::new();
    let mut chunk_set = HashSet::new();

    loop {
        for pos in 1..=3 {
            let char = if let Some(last) = result.last() {
                last.get(pos).expect("unreachable").clone()
            } else {
                let Some(y) = chars.next() else { break; };
                y
            };
            chunk4.push(char);
            chunk_set.insert(char);
        }

        let Some(last_char_of_chunk) = chars.next() else { break; };
        chunk4.push(last_char_of_chunk);
        chunk_set.insert(last_char_of_chunk);
        result.push(chunk4.clone());

        if chunk_set.len() == 4 {
            break;
        }

        chunk4 = Vec::new();
        chunk_set = HashSet::new();
    }

    result
        .iter()
        .map(|chars| String::from_iter(chars.iter()))
        .collect()
}

pub fn first_4chunk_without_repetition(text: String) -> Vec<String> {
    let chunks_raw: Vec<Vec<char>> = text.chars().fold(Vec::new(), |acc, char| {
        let (mut less4, mut size4): (Vec<Vec<char>>, Vec<Vec<char>>) =
            acc.into_iter().partition(|it| it.len() < 4);

        if let Some(last) = size4.iter().last() {
            let duplicates_removed: HashSet<&char> = HashSet::from_iter(last.iter());
            if duplicates_removed.len() == 4 {
                return size4;
            }
        }

        less4.iter_mut().for_each(|item| item.push(char));
        less4.push(vec![char]);
        size4.extend(less4);
        size4
    });

    chunks_raw
        .into_iter()
        .map(|chars| String::from_iter(chars.iter()))
        .collect()
}
