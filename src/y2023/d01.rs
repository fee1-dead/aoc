use crate::*;

pub fn part1(s: String) -> Result<()> {
    let n: u32 = s
        .lines()
        .map(|s| {
            let mut x = s.chars().filter_map(|c| c.to_digit(10));

            x.clone().next().unwrap() * 10 + x.next_back().unwrap()
        })
        .sum();

    dbg!(n);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let n: usize = s
        .lines()
        .map(|s| {
            let a = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            let mut ten = None;
            let mut one = None;
            for index in 0..s.len() {
                let substr = &s[index..];
                if let Some(digit) = (substr.as_bytes()[0] as char).to_digit(10) {
                    ten = Some(digit as usize);
                    break;
                } else if let Some(i) = a.into_iter().position(|x| substr.starts_with(x)) {
                    ten = Some(i + 1);
                    break;
                }
            }

            for index in (0..s.len()).rev() {
                let substr = &s[..=index];
                if let Some(digit) = substr.chars().next_back().unwrap().to_digit(10) {
                    one = Some(digit as usize);
                    break;
                } else if let Some(i) = a.into_iter().position(|x| substr.ends_with(x)) {
                    one = Some(i + 1);
                    break;
                }
            }

            ten.unwrap() * 10 + one.unwrap()
        })
        .sum();
    dbg!(n);
    Ok(())
}
