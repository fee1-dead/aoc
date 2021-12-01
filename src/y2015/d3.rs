use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut map = HashSet::default();
    map.insert((0, 0));

    let mut x = 0;
    let mut y = 0;
    for c in s.bytes() {
        match c {
            b'^' => y += 1,
            b'v' => y -= 1,
            b'>' => x += 1,
            b'<' => x -= 1,
            c => bail!("invalid byte {}", c),
        }
        map.insert((x, y));
    }

    dbg!(map.len());

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut map = HashSet::default();
    map.insert((0, 0));

    let mut robo = false;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;

    for c in s.bytes() {
        let (x, y) = if robo {
            (&mut x1, &mut y1)
        } else {
            (&mut x2, &mut y2)
        };

        match c {
            b'^' => *y += 1,
            b'v' => *y -= 1,
            b'>' => *x += 1,
            b'<' => *x -= 1,
            c => bail!("invalid byte {}", c),
        }
        map.insert((*x, *y));

        robo = !robo;
    }

    dbg!(map.len());

    Ok(())
}
