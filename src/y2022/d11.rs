use std::mem::take;
use std::ops::{Add, Mul};

use crate::*;

#[derive(Debug)]
enum Op {
    Square,
    Mul(u64),
    Add(u64),
}

#[derive(Clone)]
pub struct Modulo {
    by: HashMap<u64, u64>,
}

impl Mul for Modulo {
    type Output = Self;

    fn mul(self, mut rhs: Self) -> Self::Output {
        for (k, v) in self.by {
            *rhs.by.get_mut(&k).unwrap() *= v;
            *rhs.by.get_mut(&k).unwrap() %= k;
        }
        rhs
    }
}

impl Mul<u64> for Modulo {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        let mut by = HashMap::default();
        for (k, v) in self.by {
            by.insert(k, v * rhs % k);
        }
        Self { by }
    }
}

impl Add<u64> for Modulo {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        let mut by = HashMap::default();
        for (k, v) in self.by {
            by.insert(k, (v + rhs) % k);
        }
        Self { by }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Op,
    test_div: u64,
    if_true_throw: usize,
    if_false_throw: usize,
    ins: usize,
}

struct MonkeyMod {
    items: Vec<Modulo>,
    operation: Op,
    test_div: u64,
    if_true_throw: usize,
    if_false_throw: usize,
    ins: usize,
}

pub fn part1(s: String) -> Result<()> {
    let mut monkeys = Vec::new();
    for mut l in &s.lines().chunks(7) {
        l.next();
        let items = l
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut op = l
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap();
        let op = if op == "* old" {
            Op::Square
        } else if op.starts_with('*') {
            op = op.strip_prefix("* ").unwrap();
            Op::Mul(op.parse::<u64>().unwrap())
        } else {
            op = op.strip_prefix("+ ").unwrap();
            Op::Add(op.parse::<u64>().unwrap())
        };
        let test_div = l
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()?;
        let if_true_throw = l
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()?;
        let if_false_throw = l
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()?;
        let m = Monkey {
            items,
            operation: op,
            test_div,
            if_true_throw,
            if_false_throw,
            ins: 0,
        };
        monkeys.push(m);
    }
    dbg!(&monkeys);

    for _ in 0..20 {
        for monke in 0..monkeys.len() {
            for item in take(&mut monkeys[monke].items) {
                let mut new_item = match monkeys[monke].operation {
                    Op::Mul(x) => item * x,
                    Op::Add(x) => item + x,
                    Op::Square => item * item,
                };
                new_item /= 3;

                let if_true = monkeys[monke].if_true_throw;
                let if_false = monkeys[monke].if_false_throw;
                if new_item % monkeys[monke].test_div == 0 {
                    monkeys[if_true].items.push(new_item);
                } else {
                    monkeys[if_false].items.push(new_item);
                }
                monkeys[monke].ins += 1;
            }
        }
    }
    monkeys.sort_unstable_by(|a, b| a.ins.cmp(&b.ins));

    dbg!(monkeys
        .iter()
        .rev()
        .map(|m| m.ins)
        .take(2)
        .product::<usize>());
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut monkeys = Vec::new();
    for mut l in &s.lines().chunks(7) {
        l.next();
        let items = l
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut op = l
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap();
        let op = if op == "* old" {
            Op::Square
        } else if op.starts_with('*') {
            op = op.strip_prefix("* ").unwrap();
            Op::Mul(op.parse::<u64>().unwrap())
        } else {
            op = op.strip_prefix("+ ").unwrap();
            Op::Add(op.parse::<u64>().unwrap())
        };
        let test_div = l
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()?;
        let if_true_throw = l
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()?;
        let if_false_throw = l
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()?;
        let m = Monkey {
            items,
            operation: op,
            test_div,
            if_true_throw,
            if_false_throw,
            ins: 0,
        };
        monkeys.push(m);
    }
    let test_divs = monkeys.iter().map(|m| m.test_div).collect_vec();

    let mut monkeys = monkeys
        .into_iter()
        .map(|m| MonkeyMod {
            items: m
                .items
                .into_iter()
                .map(|x| Modulo {
                    by: test_divs
                        .iter()
                        .copied()
                        .map(|modulo| (modulo, x % modulo))
                        .collect(),
                })
                .collect(),
            operation: m.operation,
            test_div: m.test_div,
            if_true_throw: m.if_true_throw,
            if_false_throw: m.if_false_throw,
            ins: 0,
        })
        .collect::<Vec<_>>();

    for _ in 0..10000 {
        for monke in 0..monkeys.len() {
            for item in take(&mut monkeys[monke].items) {
                let new_item = match monkeys[monke].operation {
                    Op::Mul(x) => item * x,
                    Op::Add(x) => item + x,
                    Op::Square => item.clone() * item.clone(),
                };

                let if_true = monkeys[monke].if_true_throw;
                let if_false = monkeys[monke].if_false_throw;
                if new_item.by[&monkeys[monke].test_div] == 0 {
                    monkeys[if_true].items.push(new_item);
                } else {
                    monkeys[if_false].items.push(new_item);
                }
                monkeys[monke].ins += 1;
            }
        }
    }
    monkeys.sort_unstable_by(|a, b| a.ins.cmp(&b.ins));

    dbg!(monkeys
        .iter()
        .rev()
        .map(|m| m.ins)
        .take(2)
        .product::<usize>());
    Ok(())
}
