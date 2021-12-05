use itertools::Either;

use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut grid = [0u8; 1000 * 1000];
    for l in s.lines() {
        let x1: usize;
        let y1: usize;
        let x2: usize;
        let y2: usize;
        scanfmt!(l, "{},{} -> {},{}", x1, y1, x2, y2);

        if x1 == x2 {
            let min = y1.min(y2);
            let max = y1.max(y2);
            for y in min..=max {
                grid[y * 1000 + x1] += 1;
            }
        } else if y1 == y2 {
            let min = x1.min(x2);
            let max = x1.max(x2);
            for x in min..=max {
                grid[y1 * 1000 + x] += 1;
            }
        }
    }

    dbg!(grid.iter().filter(|n| **n >= 2).count());

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut grid = [0u8; 1000 * 1000];
    for l in s.lines() {
        let x1: usize;
        let y1: usize;
        let x2: usize;
        let y2: usize;
        scanfmt!(l, "{},{} -> {},{}", x1, y1, x2, y2);

        if x1 == x2 {
            let min = y1.min(y2);
            let max = y1.max(y2);
            for y in min..=max {
                grid[y * 1000 + x1] += 1;
            }
        } else if y1 == y2 {
            let min = x1.min(x2);
            let max = x1.max(x2);
            for x in min..=max {
                grid[y1 * 1000 + x] += 1;
            }
        } else {
            assert_eq!(x1.abs_diff(x2), y1.abs_diff(y2));

            let xs = if x1 < x2 {
                Either::Left(x1..=x2)
            } else {
                Either::Right((x2..=x1).rev())
            };

            let ys = if y1 < y2 {
                Either::Left(y1..=y2)
            } else {
                Either::Right((y2..=y1).rev())
            };

            for (x, y) in xs.zip(ys) {
                grid[y * 1000 + x] += 1;
            }
        }
    }

    dbg!(grid.iter().filter(|n| **n >= 2).count());

    Ok(())
}
