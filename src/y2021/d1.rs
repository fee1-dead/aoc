use crate::*;

pub fn part1(s: String) -> Result<()> {
    let integers: Vec<usize> = s.lines().map(str::parse).collect::<Result<_, _>>()?;
    let inc = integers.windows(2).filter(|v| v[1] > v[0]).count();

    dbg!(inc);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let integers: Vec<usize> = s.lines().map(str::parse).collect::<Result<_, _>>()?;
    let inc = integers.windows(4).filter(|v| v[3] > v[0]).count();

    dbg!(inc);

    Ok(())
}
