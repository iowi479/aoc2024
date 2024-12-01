mod day01;

use crate::challenge;
use anyhow::Result;

type Challenge = Box<dyn challenge::Challenge>;

pub fn run(day: u32) -> Result<()> {
    let challenge = find_challenge(day)?;
    let example_path = format!("input/day{:02}_example.txt", day);
    let default_path = format!("input/day{:02}.txt", day);

    if std::fs::exists(&example_path)? {
        let input = std::fs::read_to_string(example_path)?;
        println!("Example input:");
        let part1 = challenge.part1(&input)?;
        println!("Part 1: {}", part1);

        let part2_res = challenge.part2(&input);
        let part2 = match part2_res {
            Ok(part2) => part2,
            Err(e) => e.to_string(),
        };

        println!("Part 2: {}", part2);

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    if std::fs::exists(&default_path)? {
        let input = std::fs::read_to_string(default_path)?;
        let part1 = challenge.part1(&input)?;
        println!("Part 1: {}", part1);

        let part2 = challenge.part2(&input)?;
        println!("Part 2: {}", part2);

        return Ok(());
    } else {
        let input1 = std::fs::read_to_string(format!("input/day{:02}_1.txt", day))?;
        let part1 = challenge.part1(&input1)?;
        println!("Part 1: {}", part1);

        let input2 = std::fs::read_to_string(format!("input/day{:02}_2.txt", day))?;
        let part2 = challenge.part2(&input2)?;
        println!("Part 2: {}", part2);

        return Ok(());
    }
}

fn find_challenge(day: u32) -> Result<Challenge> {
    match day {
        1 => Ok(Box::new(day01::Day01::new())),
        _ => Err(anyhow::anyhow!("Day {} not implemented yet", day)),
    }
}
