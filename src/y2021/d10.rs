use num::bigint::ToBigUint;

use crate::*;

fn calc(ch: u8) -> usize {
    match ch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!(),
    }
}

pub fn part1(s: String) -> Result<()> {
    let mut stack = vec![];
    let mut total = 0;

    for l in s.lines() {
        for ch in l.bytes() {
            match ch {
                b'(' => stack.push(b')'),
                b'<' => stack.push(b'>'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                ch => {
                    if stack.pop() != Some(ch) {
                        total += calc(ch);
                        break;
                    }
                }
            }
        }
    }

    dbg!(total);
    todo!()
}

pub fn part2(s: String) -> Result<()> {
    let mut scores = vec![];

    'outer: for l in s.lines() {
        let mut stack = vec![];

        for ch in l.bytes() {
            match ch {
                b'(' => stack.push(b')'),
                b'<' => stack.push(b'>'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                ch => {
                    if stack.pop() != Some(ch) {
                        continue 'outer;
                    }
                }
            }
        }

        let mut score: num::BigUint = 0u8.into();
        while let Some(ch) = stack.pop() {
            score *= 5u8.to_biguint().unwrap();
            score += match ch {
                b')' => 1usize,
                b']' => 2,
                b'}' => 3,
                b'>' => 4,
                _ => panic!(),
            }.to_biguint().unwrap();
        }
        scores.push(score);
    }

    scores.sort_unstable();
    dbg!(&scores[(scores.len() - 1) / 2]);
    Ok(())
}