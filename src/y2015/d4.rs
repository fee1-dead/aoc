use crate::*;

pub fn part1(s: String) -> Result<()> {
    let s = s.trim();
    for i in 1.. {
        let digest = md5::compute(format!("{s}{i}"));
        if digest[0] != 0 {
            continue;
        }
        if digest[1] != 0 {
            continue;
        }
        if digest[2].leading_zeros() >= 4 {
            dbg!(i);
            return Ok(());
        }
    }
    unreachable!()
}

pub fn part2(s: String) -> Result<()> {
    let s = s.trim();
    for i in 1.. {
        let digest = md5::compute(format!("{s}{i}"));
        if digest[0] != 0 {
            continue;
        }
        if digest[1] != 0 {
            continue;
        }
        if digest[2] == 0 {
            dbg!(i);
            return Ok(());
        }
    }
    unreachable!()
}
