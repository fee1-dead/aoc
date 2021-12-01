use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut lights = [[false; 1000]; 1000];
    for l in s.lines() {
        let instruction1: String;
        let instruction2: String;
        let x1: usize;
        let y1: usize;
        let x2: usize;
        let y2: usize;
        scanfmt!(
            l,
            "{} {}{},{} through {},{}",
            instruction1,
            instruction2,
            x1,
            y1,
            x2,
            y2
        );
        match (&*instruction1, &*instruction2) {
            ("toggle", "") => {
                for column in lights.iter_mut().skip(x1).take(x2 + 1) {
                    for light in column.iter_mut().skip(y1).take(y2 + 1) {
                        *light = !*light;
                    }
                }
            }
            ("turn", state) => {
                let state = state != "off ";

                for column in lights.iter_mut().skip(x1).take(x2 + 1) {
                    for light in column.iter_mut().skip(y1).take(y2 + 1) {
                        *light = state;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let count: usize = lights
        .into_iter()
        .map(|lights| lights.into_iter().filter(|b| *b).count())
        .sum();
    dbg!(count);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut lights = [[0u32; 1000]; 1000];
    for l in s.lines() {
        let instruction1: String;
        let instruction2: String;
        let x1: usize;
        let y1: usize;
        let x2: usize;
        let y2: usize;
        scanfmt!(
            l,
            "{} {}{},{} through {},{}",
            instruction1,
            instruction2,
            x1,
            y1,
            x2,
            y2
        );
        match (&*instruction1, &*instruction2) {
            ("toggle", "") => {
                for column in lights.iter_mut().skip(x1).take(x2 + 1) {
                    for light in column.iter_mut().skip(y1).take(y2 + 1) {
                        *light += 2;
                    }
                }
            }
            ("turn", "off ") => {
                for column in lights.iter_mut().skip(x1).take(x2 + 1) {
                    for light in column.iter_mut().skip(y1).take(y2 + 1) {
                        *light = light.saturating_sub(1);
                    }
                }
            }
            ("turn", "on ") => {
                for column in lights.iter_mut().skip(x1).take(x2 + 1) {
                    for light in column.iter_mut().skip(y1).take(y2 + 1) {
                        *light += 1;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let count: u32 = lights
        .into_iter()
        .map(|lights| lights.into_iter().sum::<u32>())
        .sum();
    dbg!(count);

    Ok(())
}
