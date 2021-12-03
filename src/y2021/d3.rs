use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut cnt = [0; 12];
    let mut count = 0;

    for l in s.lines() {
        for (n, ch) in l.chars().enumerate() {
            if ch == '1' {
                cnt[n] += 1;
            }
        }
        count += 1;
    }

    let gamma = cnt.into_iter().map(|cnt| if cnt > (count / 2) {
        '1'
    } else {
        '0'
    }).collect::<String>();

    let gamma = u32::from_str_radix(&gamma, 2)?;
    let epsilon = (!gamma) & 0b1111_1111_1111;

    dbg!(gamma * epsilon);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let num = s.lines().map(|l| u32::from_str_radix(l, 2)).collect::<Result<Vec<_>, _>>()?;
        
    let mut oxy = num;
    let mut carbon = oxy.clone();

    for bit in (0..12).rev() {
        let bt = 1 << bit;

        let set = |list: &mut Vec<u32>, flip| {
            let ones = list.iter().filter(|n| (**n & bt) != 0).count();

            if list.len() == 1 { return }

            let mut is_one = ones >= (list.len() - ones);
            is_one ^= flip;
            let criteria = (is_one as u32) << bit;
            list.retain(|&test| (test & bt) ^ criteria == 0);
        };

        set(&mut oxy, false);
        set(&mut carbon, true);
    }

    assert_eq!(1, oxy.len());
    assert_eq!(1, carbon.len());

    dbg!(oxy[0] * carbon[0]);

    Ok(())
}