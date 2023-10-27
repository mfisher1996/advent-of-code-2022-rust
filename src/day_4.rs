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
                let (min1,max1) = if x[0].0 > x[0].1 { (x[0].1,x[0].0) } else { x[0] };
                let (min2,max2) = if x[1].0 > x[1].1 { (x[1].1,x[1].0) } else { x[1] };
                
                // 1-1, 1-2
                (min1 >= min2 && min1 <= max2)      // min1 is in range2
                || (max1 >= min2 && max1 <= max2)   // max1 is in range2
                || (min2 >= min1 && min2 <= max1)   // min2 is in range1
                || (max2 >= min1 && max2 <= max1)   // max2 is in range1
            }).next().unwrap()
        ).filter(|x| *x).count();
        
    println!("{}", solution);
    Ok(())
}
