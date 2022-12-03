use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut p = 0;
    for x in s.lines() {
        let (a, b) = x.split_at(x.len() / 2);
        for ch in b.chars() {
            if a.contains(ch) {
                p += match ch {
                    'a'..='z' => ch as u32 - 'a' as u32 + 1,
                    'A'..='Z' => ch as u32 - 'A' as u32 + 27,
                    _ => todo!(),
                };
                break;
            }
        }
    }
    dbg!(p);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut p = 0;
    for mut x in &s.lines().chunks(3) {
        let mut set: HashSet<_> = x.next().unwrap().chars().collect();
        for chunk in x {
            set.retain(|ch| chunk.contains(*ch));
        }
        let ch = set.into_iter().next().unwrap();
        p += match ch {
            'a'..='z' => ch as u32 - 'a' as u32 + 1,
            'A'..='Z' => ch as u32 - 'A' as u32 + 27,
            _ => todo!(),
        };
    }
    dbg!(p);
    Ok(())
}
