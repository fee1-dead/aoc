use crate::*;

pub fn part1(s: String) -> Result<()> {
    dbg!(
        s.as_bytes()
            .windows(4)
            .enumerate()
            .find(|(_, x)| x.iter().duplicates().next().is_none())
            .unwrap()
            .0
            + 4
    );
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    dbg!(
        s.as_bytes()
            .windows(14)
            .enumerate()
            .find(|(_, x)| x.iter().duplicates().next().is_none())
            .unwrap()
            .0
            + 14
    );

    Ok(())
}
