use std::convert::identity;

use crate::*;

fn solve(s: String, map_fn: fn(usize) -> usize) -> Result<()> {
    let mut cost = 0;
    let mut target = 0;

    let positions: Vec<isize> = s.split(',').map(str::parse).map(Result::unwrap).collect();

    loop {
        let mut oldcost = 0;
        let mut newcost = 0;
        for pos in positions.iter() {
            newcost += map_fn(pos.abs_diff(target + 1));
            oldcost += map_fn(pos.abs_diff(target));
        }

        if newcost < oldcost {
            target += 1;
            cost = newcost;
        } else {
            break;
        }
    }

    dbg!(target);
    dbg!(cost);

    Ok(())
}

pub fn part1(s: String) -> Result<()> {
    solve(s, identity)
}

// http://oeis.org/A000217
fn t(n: usize) -> usize {
    n * (n + 1) / 2
}

pub fn part2(s: String) -> Result<()> {
    solve(s, t)
}