use crate::*;
pub fn part1(s: String) -> Result<()> {
    let mut i = 0;
    for c in s.bytes() {
        match c {
            b'(' => i += 1,
            b')' => i -= 1,
            c => bail!("invalid byte {}", c),
        }
    }

    dbg!(i);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut i = 0;
    for (index, c) in s.bytes().enumerate() {
        match c {
            b'(' => i += 1,
            b')' => i -= 1,
            _ => unreachable!(),
        }

        if i == -1 {
            dbg!(index + 1);
            return Ok(());
        }
    }

    unreachable!()
}
