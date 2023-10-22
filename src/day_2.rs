pub fn part_1() -> Result<(), Box<dyn std::error::Error>> {
    let solution: i32 = std::fs::read_to_string("input/day_2.prod")?
        .split("\n")
        .map(|plays| plays
            .chars()
            .filter_map(|x| match x {
                'X'|'A' => Some(1),
                'Y'|'B' => Some(2),
                'Z'|'C' => Some(3),
                _ => None,
            })
            .collect::<Vec<i32>>()
            .chunks(2)
            .map(|x| match x[0] - x[1] {
                0 => 3 + x[0],
                1 | -2 => x[1],
                _ => 6 + x[1],
            })
            .sum::<i32>()
        ).sum();
    println!("{}", solution);
    Ok(())
}

pub fn part_2() -> Result<(), Box<dyn std::error::Error>> {
    let solution: i32 = std::fs::read_to_string("input/day_2.prod")?
        .split("\n")
        .map(|plays| plays
            .chars()
            .filter_map(|x| match x {
                'X'|'A' => Some(1),
                'Y'|'B' => Some(2),
                'Z'|'C' => Some(3),
                _ => None,
            })
            .collect::<Vec<i32>>()
            .chunks(2)
            .map(|x| match x[1] {
                1 if x[0] == 1 => 3,    // loose
                1 => x[0] - 1,          // loose
                2 => 3 + x[0],          // draw
                3 if x[0] == 3 => 7,    // win
                3 => x[0] + 7,          // win
                _ => 0
            })
            .sum::<i32>()
        ).sum();
    println!("{}", solution);
    Ok(())
}
