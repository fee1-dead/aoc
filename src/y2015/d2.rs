use crate::*;
pub fn part1(s: String) -> Result<()> {
    let mut total = 0;
    let mut x: u64;
    let mut y: u64;
    let mut z: u64;

    for l in s.lines() {
        scanfmt!(l, "{}x{}x{}", x, y, z);
        total += (x * y + y * z + x * z) * 2;
        let max = x.max(y).max(z);
        total += (x * y * z) / max;
    }

    dbg!(total);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut total = 0;
    let mut x: u64;
    let mut y: u64;
    let mut z: u64;

    for l in s.lines() {
        scanfmt!(l.trim(), "{}x{}x{}", x, y, z);
        let max = x.max(y).max(z);
        let sum_of_smallest_sides = x + y + z - max;
        let perimeter_of_smallest_face = sum_of_smallest_sides * 2;
        let volume = x * y * z;
        total += perimeter_of_smallest_face + volume;
    }

    dbg!(total);

    Ok(())
}
