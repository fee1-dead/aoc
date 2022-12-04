use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut n = 0;
    for l in s.lines() {
        let a: u32; let b : u32; let c: u32; let d: u32;
        scanfmt!(l, "{}-{},{}-{}", a, b, c, d);
        if (a >= c && b <= d) || (c >= a && d <= b) {
            n+= 1;
        }
    }
    dbg!(n);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut n = 0;
    for l in s.lines() {
        let a: u32; let b : u32; let c: u32; let d: u32;
        scanfmt!(l, "{}-{},{}-{}", a, b, c, d);
        if (a <= d && b >= c) || (c <= b && d >= a) {
            n += 1;
        }
    }
    dbg!(n);
    Ok(())
}
