pub struct Year {
    pub year: u16,
    pub days: &'static [Day],
}

pub struct Day {
    pub part1: fn(String) -> crate::Result<()>,
    pub part2: fn(String) -> crate::Result<()>,
}
