use anyhow::Result;

pub trait Challenge {
    fn part1(&self, input: &str) -> Result<String> {
        Err(anyhow::anyhow!("Part 1 not implemented"))
    }

    fn part2(&self, input: &str) -> Result<String> {
        Err(anyhow::anyhow!("Part 2 not implemented"))
    }
}
