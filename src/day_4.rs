pub fn part_1() -> Result<(), Box<dyn std::error::Error>> {
    let solution = std::fs::read_to_string("input/day_4.prod")?
        .lines()
        .map(|line|  line                            
                .split(',')
                .map(|range| {
                    let mut x = range
                        .split('-')
                        .map(|x| x.parse::<i32>().unwrap()) ;
                    (x.next().unwrap(), x.next().unwrap())
                }).collect::<Vec<(i32, i32)>>()
            .chunks(2)
            .map(|x| {
                let (min1,max1) = x[0];
                let (min2,max2) = x[1];
                (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
            }).next().unwrap()
        ).filter(|x| *x).count();
        
    println!("{}", solution);
    Ok(())
}

pub fn part_2() -> Result<(), Box<dyn std::error::Error>> {
    let solution = "0".to_string();
    println!("{}", solution);
    Ok(())
}
