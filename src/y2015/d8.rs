use crate::*;

pub fn part1(s: String) -> Result<()> {
    fn scan(s: &str, count: &mut usize) {
        *count += 2;
        let s = &s[1..s.len() - 1];
        let mut bytes = s.bytes();

        while let Some(b) = bytes.next() {
            if b == b'\\' {
                match bytes.next() {
                    Some(b'"' | b'\\') => *count += 1,
                    Some(b'x') => {
                        bytes.advance_by(2).unwrap();
                        *count += 3;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    let mut c = 0;
    for l in s.lines() {
        scan(l, &mut c);
    }
    dbg!(c);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    fn scan(s: &str, count: &mut usize) {
        *count += 4;
        let s = &s[1..s.len() - 1];
        let bytes = s.bytes();

        for b in bytes {
            match b {
                b'\\' | b'"' => *count += 1,
                _ => {}
            }
        }
    }

    let mut c = 0;
    for l in s.lines() {
        scan(l, &mut c);
    }
    dbg!(c);

    Ok(())
}
