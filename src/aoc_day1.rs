pub fn group_food_calories_by_elf(calories_list: Vec<String>) -> Vec<usize> {
    let clusters: Vec<&[String]> = calories_list.split(|e| e.to_string() == "").collect();
    let mut result = Vec::new();
    for cluster in clusters {
        result.push(
            cluster
                .into_iter()
                .map(|e| e.to_string().parse::<usize>().unwrap())
                .sum(),
        );
    }
    result
}

pub fn max_elements(calories_list: Vec<usize>, to_pick: usize) -> Vec<usize> {
    let mut maximums = vec![0; to_pick];
    let mut min: &mut usize = maximums.get_mut(0).unwrap();

    for calories in calories_list {
        if *min < calories {
            *min = calories;
            min = maximums.iter_mut().min().unwrap();
        }
    }
    maximums
}

pub fn solution(calories_list: Vec<String>) -> usize {
    let calories_of_each_elf = group_food_calories_by_elf(calories_list);
    calories_of_each_elf.into_iter().max().unwrap()
}

pub fn solution_part2(calories_list: Vec<String>) -> usize {
    let calories_of_each_elf = group_food_calories_by_elf(calories_list);
    max_elements(calories_of_each_elf, 3).iter().sum()
}
