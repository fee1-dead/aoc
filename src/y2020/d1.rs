use crate::*;

pub fn part1(s: String) -> Result<()> {
    let integers: Vec<usize> = s.lines().map(str::parse).collect::<Result<_, _>>()?;
    for &a in &integers {
        for &b in &integers {
            if a + b == 2020 {
                dbg!(a * b);
            }
        }
    }
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let integers: Vec<usize> = s.lines().map(str::parse).collect::<Result<_, _>>()?;
    for &a in &integers {
        for &b in &integers {
            for &c in &integers {
                if a + b + c == 2020 {
                    dbg!(a * b * c);
                }
            }
        }
    }
    Ok(())
}
