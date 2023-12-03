use std::cell::Cell;

use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut sum = 0;
    let lines = s.lines().collect::<Vec<_>>();
    for (ln, line) in lines.iter().copied().enumerate() {
        let num_cnt = Cell::new(0);
        let mut check_num = |cn: usize, char| {
            if num_cnt.get() != 0 {
                let start = cn - num_cnt.get();
                let mut is_adj = false;
                if char != '.'
                    || start
                        .checked_sub(1)
                        .and_then(|i| line.as_bytes().get(i))
                        .map_or(false, |&x| !(x as char).is_ascii_digit() && x != b'.')
                {
                    is_adj = true;
                }
                let range = start.saturating_sub(1) ..= cn.min(line.len() - 1);
                if [
                    ln.checked_sub(1).and_then(|l| lines.get(l)),
                    lines.get(ln + 1),
                ]
                .into_iter()
                .any(|line| {
                    line.map_or(false, |x| {
                        x[range.clone()]
                            .chars()
                            .any(|x| !x.is_ascii_digit() && x != '.')
                    })
                }) {
                    is_adj = true;
                }
                if is_adj {
                    let s = &line[start..cn];
                    sum += s.parse::<usize>().unwrap();
                }
                num_cnt.set(0);
            }
        };
        for (cn, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                num_cnt.set(num_cnt.get() + 1);
            } else {
                check_num(cn, char);
            }
        }
        check_num(line.len(), '.');
    }
    dbg!(sum);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut sum = 0;
    let lines = s.lines().collect::<Vec<_>>();
    let mut gears: HashMap<(usize, usize), (usize, usize)> = HashMap::default();
    for (ln, line) in lines.iter().copied().enumerate() {
        let num_cnt = Cell::new(0);
        let mut check_num = |cn: usize, char| {
            if num_cnt.get() != 0 {
                let start = cn - num_cnt.get();
                let s = &line[start..cn];
                let num = s.parse::<usize>().unwrap();
                if char == '*' {
                    let (count, ratio) = gears.entry((ln, cn)).or_insert((0, 1));
                    *count += 1;
                    *ratio *= num;
                }
                if let Some(i) = start.checked_sub(1) {
                    if line.as_bytes()[i] == b'*' {
                        let (count, ratio) = gears.entry((ln, i)).or_insert((0, 1));
                        *count += 1;
                        *ratio *= num;
                    }
                }
                for line in ln.checked_sub(1).into_iter().chain(Some(ln + 1)) {
                    let Some(line_str) = lines.get(line) else { continue };
                    let range = start.saturating_sub(1) ..= cn.min(line_str.len() - 1);
                    for (idx, _) in line_str.as_bytes()[range.clone()].iter().enumerate().filter(|&(_, &x)| x == b'*') {
                        let (count, ratio) = gears.entry((line, idx + range.start())).or_insert((0, 1));
                        *count += 1;
                        *ratio *= num;
                    }
                }
                num_cnt.set(0);
            }
        };
        for (cn, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                num_cnt.set(num_cnt.get() + 1);
            } else {
                check_num(cn, char);
            }
        }
        check_num(line.len(), '.');
    }
    //let mut pos = gears.iter().filter(|(_, (count, _))| *count == 2).map(|(pos, (_, num))| (pos, num)).collect_vec();
    //pos.sort();
    //pos.into_iter().for_each(|x| println!("{x:?}"));
    for (_, (count, ratio)) in gears {
        if count != 2 { continue };
        sum += ratio;
    }
    dbg!(sum);
    Ok(())
}
