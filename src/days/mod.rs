mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crate::challenge;
use anyhow::Result;

type Challenge = Box<dyn challenge::Challenge>;

/// Run the challenge for the given day.
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
    } else {
        let example1_path = format!("input/day{:02}_example1.txt", day);
        let example2_path = format!("input/day{:02}_example2.txt", day);

        if std::fs::exists(&example1_path)? && std::fs::exists(&example2_path)? {
            let input1 = std::fs::read_to_string(example1_path)?;
            println!("Example input 1:");
            let part1 = challenge.part1(&input1)?;
            println!("Part 1: {}", part1);

            let input2 = std::fs::read_to_string(example2_path)?;
            println!("Example input 2:");
            let part2 = challenge.part2(&input2)?;
            println!("Part 2: {}", part2);

            std::thread::sleep(std::time::Duration::from_secs(2));
        }
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

/// Creates an Instance of the challenge for the given day.
fn find_challenge(day: u32) -> Result<Challenge> {
    match day {
        1 => Ok(Box::new(day01::Day01::new())),
        2 => Ok(Box::new(day02::Day02::new())),
        3 => Ok(Box::new(day03::Day03::new())),
        4 => Ok(Box::new(day04::Day04::new())),
        5 => Ok(Box::new(day05::Day05::new())),
        6 => Ok(Box::new(day06::Day06::new())),
        7 => Ok(Box::new(day07::Day07::new())),
        8 => Ok(Box::new(day08::Day08::new())),
        9 => Ok(Box::new(day09::Day09::new())),
        10 => Ok(Box::new(day10::Day10::new())),
        11 => Ok(Box::new(day11::Day11::new())),
        12 => Ok(Box::new(day12::Day12::new())),
        13 => Ok(Box::new(day13::Day13::new())),
        14 => Ok(Box::new(day14::Day14::new())),
        15 => Ok(Box::new(day15::Day15::new())),
        16 => Ok(Box::new(day16::Day16::new())),
        17 => Ok(Box::new(day17::Day17::new())),
        18 => Ok(Box::new(day18::Day18::new())),
        19 => Ok(Box::new(day19::Day19::new())),
        20 => Ok(Box::new(day20::Day20::new())),
        21 => Ok(Box::new(day21::Day21::new())),
        22 => Ok(Box::new(day22::Day22::new())),
        23 => Ok(Box::new(day23::Day23::new())),
        24 => Ok(Box::new(day24::Day24::new())),
        25 => Ok(Box::new(day25::Day25::new())),
        _ => Err(anyhow::anyhow!("Day {} is not in Advent of Code", day)),
    }
}
