pub fn part_1() -> Result<(), Box<dyn std::error::Error>> {
    let solution: i32 = std::fs::read_to_string("input/day_3.prod")?
        .lines()
        .map(|line| line .split_at(line.len() / 2))
        .map(|(first, second)| first.chars().find(|c| second.contains(*c)).unwrap())
        .map(|c| {
            if c as i32 > 'Z' as i32 {
                c as i32 - 'a' as i32 + 1
            } else {
                c as i32 - 'A' as i32 + 27 
            }
        })
        .sum();
    println!("{}", solution);
    Ok(())
}

pub fn part_2() -> Result<(), Box<dyn std::error::Error>> {
    let solution: i32 = std::fs::read_to_string("input/day_3.prod")?
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|x| x[0].chars().find(|c| x[1].contains(*c) && x[2].contains(*c)).unwrap())
        .map(|c| {
            if c as i32 > 'Z' as i32 {
                c as i32 - 'a' as i32 + 1
            } else {
                c as i32 - 'A' as i32 + 27 
            }
        })
        .sum();
    println!("{}", solution);
    Ok(())
}
