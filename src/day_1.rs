pub fn part_1() -> Result<(), Box<dyn std::error::Error>> {
    let solution: i32 = std::fs::read_to_string("input/day_1.prod")?
        .split("\n\n")
        .map(|elf| 
            elf.split("\n").map(|cal| cal.parse::<i32>().unwrap_or(0)
            ).sum::<i32>())
        .max().unwrap();
    println!("{}", solution);
    Ok(())
}

pub fn part_2() -> Result<(), Box<dyn std::error::Error>> {
    let mut sorted: Vec<i32> = std::fs::read_to_string("input/day_1.prod")?
        .split("\n\n")
        .map(|elf| 
            elf.split("\n").map(|cal| cal.parse::<i32>().unwrap_or(0)
            ).sum()
        ).collect::<Vec<i32>>();
    sorted.sort();
    let solution:i32 = sorted.into_iter().rev().take(3).sum();

    println!("{}", solution);
    Ok(())
}
