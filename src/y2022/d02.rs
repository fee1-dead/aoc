use crate::*;

pub fn part1(s: String) -> Result<()> {
    let x: u64 = s
        .lines()
        .filter_map(|s| s.split_once(" "))
        .map(|(a, b)| (a.as_bytes()[0] - b'A', b.as_bytes()[0] - b'X'))
        .map(|(a, b)| {
            (match (a, b) {
                (0, 1) | (1, 2) | (2, 0) => 6,
                (a, b) if a == b => 3,
                _ => 0,
            }) + b as u64
                + 1
        })
        .sum();
    dbg!(x);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let x: u64 = s
        .lines()
        .filter_map(|s| s.split_once(" "))
        .map(|(a, b)| (a.as_bytes()[0] - b'A', b.as_bytes()[0] - b'X'))
        .map(|(a, b)| {
            (match (a, b) {
                (a, 2) => ((a + 1) % 3) + 1,
                (a, 1) => a + 1,
                (0, 0) => 3,
                (x, 0) => x,
                _ => unreachable!(),
            }) as u64 + (b * 3) as u64
        })
    .sum();
    dbg!(x);
    Ok(())
}
