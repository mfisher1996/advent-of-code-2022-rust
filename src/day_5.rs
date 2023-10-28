use std::collections::VecDeque;
pub fn part_1() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day_5.prod")?;
    let mut stack = Cargo::from_lines(input.lines().take_while(|l| l.contains("[")).map(|l| l.to_string()).collect());
    let commands = input.lines().skip_while(|l| l.contains("[")).skip(2).map(|l| l.to_string());
    commands.map(|x| x.split(' ').filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<_>>()).for_each(|x| {
        if x.len() > 2 {
            stack.move_crates(x[0] as usize, (x[1] -1) as usize, (x[2] -1) as usize);
        }
    });
    stack.print_answer();
    println!();
    Ok(())
}

pub fn part_2() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day_5.prod")?;
    let mut stack = Cargo::from_lines(input.lines().take_while(|l| l.contains("[")).map(|l| l.to_string()).collect());
    let commands = input.lines().skip_while(|l| l.contains("[")).skip(2).map(|l| l.to_string());
    commands.map(|x| x.split(' ').filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<_>>()).for_each(|x| {
        if x.len() > 2 {
            stack.move_all_crates(x[0] as usize, (x[1] -1) as usize, (x[2] -1) as usize);
        }
    });
    stack.print_answer();
    println!();
    Ok(())
}

#[derive(Debug)]
struct Cargo {
    stacks: Vec<VecDeque<char>>,
}

impl Cargo {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut stacks:Vec<VecDeque<char>> = Vec::new();
        lines.into_iter().for_each(|line| { 
            let inbound = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if i % 4 == 1 && c != ' ' {
                    Some((i/4,c))
                } else {
                    None
                }
            }).collect::<Vec<(usize, char)>>();
            for (i, c) in inbound {
                while stacks.len() <= i {
                    stacks.push(VecDeque::new());
                }
                stacks[i].push_front(c);
            }
        });
        Self {
            stacks
        }
    }

    fn move_crates(&mut self, amount: usize, from: usize, to: usize) {
        for _ in 0..amount {
            let c = self.stacks[from].pop_back().unwrap();
            self.stacks[to]
                .push_back(c);
        }
    }
    fn move_all_crates(&mut self, amount: usize, from: usize, to: usize) {
        let c = self.stacks[from].iter().rev().take(amount).cloned().collect::<VecDeque<_>>();
        self.stacks[to].append(&mut c.into_iter().rev().collect());
        for _ in 0..amount {
            self.stacks[from].pop_back();
        }
    }
    fn print_answer(&self) {
        self.stacks.iter().for_each(|x| {
            print!("{}", x.back().unwrap());
        })
    }
}   
