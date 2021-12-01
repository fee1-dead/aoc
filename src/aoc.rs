pub struct Year {
    pub year: u16,
    pub days: &'static [Day],
}

pub struct Day {
    pub part1: fn(String) -> anyhow::Result<()>,
    pub part2: fn(String) -> anyhow::Result<()>,
}
