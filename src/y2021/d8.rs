use tap::Tap;

use crate::*;

pub fn part1(s: String) -> Result<()> {
    let cnt = s
        .lines()
        .flat_map(|l| {
            l.split(" | ")
                .last()
                .unwrap()
                .split(' ')
                .filter(|w| matches!(w.len(), 2 | 3 | 4 | 7))
        })
        .count();
    dbg!(cnt);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut sum = 0;
    for l in s.lines() {
        let mut one = None;
        let mut four = None;
        let mut seven = None;
        let mut eight = None;
        for s in l.split(' ') {
            match s.len() {
                2 => one = Some(s),
                3 => seven = Some(s),
                4 => four = Some(s),
                7 => eight = Some(s),
                _ => {}
            }
        }

        let cf = one.unwrap();
        let bd = four
            .unwrap()
            .to_owned()
            .tap_mut(|s| s.retain(|ch| !cf.contains(ch)));
        let a = seven
            .unwrap()
            .to_owned()
            .tap_mut(|s| s.retain(|ch| !cf.contains(ch)))
            .tap(|s| assert!(s.len() == 1))
            .chars()
            .next()
            .unwrap();
        let eg = eight
            .unwrap()
            .to_owned()
            .tap_mut(|s| s.retain(|ch| !cf.contains(ch)))
            .tap_mut(|s| s.retain(|ch| !bd.contains(ch)))
            .tap_mut(|s| s.retain(|ch| ch != a));
        let has_eg = |s: &str| {
            eg.chars().all(|c| s.contains(c))
        };
        let has_bd = |s: &str| {
            bd.chars().all(|c| s.contains(c))
        };
        let has_cf = |s: &str| {
            cf.chars().all(|c| s.contains(c))
        };
        let has_a = |s: &str| s.contains(a);
        
        let digits: usize = l.split_once(" | ").unwrap().1.split(' ').map(|s| {
            let digit = match s.len() {
                2 => '1',
                3 => '7',
                4 => '4',
                7 => '8',
                _ => {
                    dbg!(s);
                    let eg = has_eg(s);
                    let cf = has_cf(s);
                    let a = has_a(s);
                    let bd = has_bd(s);
                    if eg && cf && a {
                        '0'
                    } else if a && cf && bd {
                        '9'
                    } else if a && cf {
                        '3'
                    } else if a && eg && bd {
                        '6'
                    } else if a && bd {
                        '5'
                    } else if a && eg {
                        '2'
                    } else {
                        unreachable!()
                    }
                }
            };
            digit
        }).collect::<String>().parse()?;
        sum += digits;
    }
    dbg!(sum);
    Ok(())
}
