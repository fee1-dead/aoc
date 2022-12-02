use std::cmp::Ordering;
use std::ops::Not;

use crate::*;

pub fn part1(s: String) -> Result<()> {
    let x = s
        .split("\n")
        .map(|s| {
            s.is_empty()
                .not()
                .then(|| s.parse::<u64>())
                .transpose()
                .ok()
                .flatten()
        })
        .coalesce(|a, b| a.zip(b).map(|(a, b)| a + b).map(Some).ok_or((a, b)))
        .filter_map(identity)
        .max()
        .ok_or(eyre!("no max"))?;
    dbg!(x);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let x: u64 = s
        .split("\n")
        .map(|s| {
            s.is_empty()
                .not()
                .then(|| s.parse::<u64>())
                .transpose()
                .ok()
                .flatten()
        })
        .coalesce(|a, b| a.zip(b).map(|(a, b)| a + b).map(Some).ok_or((a, b)))
        .filter_map(identity)
        .sorted()
        .rev()
        .take(3)
        .sum();

    dbg!(x);

    Ok(())
}
