use std::convert::identity;
use std::num::ParseIntError;

use crate::*;

#[derive(Clone, Default)]
pub struct Board(HashMap<u32, usize>, [bool; 25], Option<u32>);

impl Board {
    fn announce(&mut self, val: u32) -> bool {
        if let Some(pos) = self.0.remove(&val) {
            self.1[pos] = true;

            if self.is_win(pos) {
                self.2 = Some(val);
                return true;
            }

            false
        } else {
            false
        }
    }
    fn is_win(&self, pos: usize) -> bool {
        let x = pos % 5;
        let y = pos / 5;

        if self.1.iter().copied().skip(x).step_by(5).all(identity) {
            return true;
        }

        if self.1.iter().copied().skip(y * 5).take(5).all(identity) {
            return true;
        }

        // diagonal
        // x _ _ _ _
        // _ x _ _ _
        // _ _ x _ _
        // _ _ _ x _
        // _ _ _ _ x
        if x == y && self.1.iter().copied().step_by(6).take(5).all(identity) {
            return true;
        }

        // anti-diagonal
        // _ _ _ _ x
        // _ _ _ x _
        // _ _ x _ _
        // _ x _ _ _
        // x _ _ _ _
        if x + y == 4
            && self
                .1
                .iter()
                .copied()
                .skip(4)
                .step_by(4)
                .take(5)
                .all(identity)
        {
            return true;
        }

        false
    }
}

fn input(
    s: &str,
) -> (
    impl Iterator<Item = Result<u32, ParseIntError>> + '_,
    Vec<Board>,
) {
    let mut boards = s.split("\n\n");
    let calls = boards.next().unwrap().split(',').map(str::parse);
    let boards = boards
        .map(|f| {
            let rows = f.lines();
            let numbers = rows
                .flat_map(|f| f.split(' ').filter_map(|s| s.parse().ok()))
                .enumerate()
                .map(|(idx, n)| (n, idx))
                .collect();
            Board(numbers, [false; 25], None)
        })
        .collect_vec();

    (calls, boards)
}

pub fn part1(s: String) -> Result<()> {
    let (calls, mut boards) = input(&s);

    for call in calls {
        let call = call?;
        if let Some(board) = boards
            .iter_mut()
            .find_map(|b| b.announce(call).then_some(b))
        {
            let sum: u32 = board.0.drain().map(|(val, _)| val).sum();
            dbg!(sum * call as u32);
            break;
        }
    }

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let (calls, mut boards) = input(&s);

    let mut last_win = None;

    for call in calls {
        let call = call?;
        boards.retain_mut(|board| {
            if board.announce(call) {
                last_win = Some(board.clone());
                false
            } else {
                true
            }
        });
    }

    let board = last_win.unwrap();
    let sum: u32 = board.0.iter().map(|(val, _)| val).sum();
    dbg!(sum * board.2.unwrap());

    Ok(())
}
