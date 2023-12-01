use std::str::FromStr;

use crate::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(i64),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => a.cmp(b),
            (Value::List(a), Value::Int(b)) => a.cmp(&vec![Value::Int(*b)]),
            (Value::Int(a), Value::List(b)) => vec![Value::Int(*a)].cmp(b),
        }
    }
}

pub fn parse_value(s: &str) -> Result<(Value, &str)> {
    if let Some(mut rest) = s.strip_prefix('[') {
        let mut list = Vec::new();
        if !rest.starts_with(']') {
            let (v, r) = parse_value(rest)?;
            rest = r;
            list.push(v);
            while let Some(r) = rest.strip_prefix(',') {
                rest = r;
                let (v, r) = parse_value(rest)?;
                rest = r;
                list.push(v);
            }
        }
        rest = rest.strip_prefix(']').unwrap();
        return Ok((Value::List(list), rest));
    } else {
        let (v, rest) = s.split_at(
            s.find(|c: char| !c.is_ascii_digit() && c != '-')
                .unwrap_or(s.len()),
        );
        return Ok((Value::Int(i64::from_str(v)?), rest));
    }
}

pub fn part1(s: String) -> Result<()> {
    let mut sum = 0;
    for (i, chunk) in s.split("\n\n").enumerate() {
        let (a, b) = chunk.split_once('\n').unwrap();
        let (a, _) = parse_value(a)?;
        let (b, _) = parse_value(b)?;
        println!("{a:?} {b:?} {:?}", a.cmp(&b));
        if a < b {
            sum += i + 1;
        }
    }
    dbg!(sum);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut packets: Vec<_> = s
        .lines()
        .filter(|x| !x.is_empty())
        .map(|s| parse_value(s).map(|(x, _)| x))
        .try_collect()?;
    let dividers = [2, 6].map(|x| Value::List(vec![Value::List(vec![Value::Int(x)])]));
    packets.extend(dividers.clone());
    packets.sort_unstable();

    let p: usize = dividers
        .into_iter()
        .map(|divider| {
            packets
                .iter()
                .position(|a| a == &divider)
                .map(|x| x + 1)
                .unwrap()
        })
        .product();
    dbg!(p);
    Ok(())
}
