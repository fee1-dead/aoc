use crate::*;

pub fn part1(s: String) -> Result<()> {
    let integers: Vec<usize> = s.lines().map(str::parse).collect::<Result<_, _>>()?;
    let inc = integers.array_windows().filter(|[a, b]| b > a).count();

    dbg!(inc);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let integers: Vec<usize> = s.lines().map(str::parse).collect::<Result<_, _>>()?;
    let inc = integers.array_windows().filter(|[a, _, _, d]| d > a).count();

    dbg!(inc);

    Ok(())
}
