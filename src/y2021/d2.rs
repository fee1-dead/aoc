use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut depth = 0;
    let mut x = 0;

    for l in s.lines() {
        let command: String;
        let rela: i32;
        scanfmt!(l, "{} {}", command, rela);
        match &*command {
            "forward" => x += rela,
            "down" => depth += rela,
            _ => depth -= rela,
        }
    }

    dbg!(x * depth);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;

    for l in s.lines() {
        let command: String;
        let rela: i32;
        scanfmt!(l, "{} {}", command, rela);
        match &*command {
            "forward" => {
                x += rela;
                depth += aim * rela
            }
            "down" => aim += rela,
            _ => aim -= rela,
        }
    }

    dbg!(x * depth);
    Ok(())
}
