use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut cycles = 0;
    let mut x = 1i64;
    let mut sum = 0i64;
    macro_rules! cycle {
        () => {
            cycles += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycles) {
                sum += x * cycles;
            }
        };
    }
    for line in s.lines() {
        if let Some((_, num)) = line.split_once("addx ") {
            let adder = num.parse::<i64>()?;
            cycle!();
            cycle!();
            x += adder;
        } else {
            cycle!();
        }
    }

    dbg!(sum);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut cycles = 0;
    let mut x = 1i64;
    macro_rules! cycle {
        () => {
            let xpos: i64 = cycles % 40;
            if xpos == 0 {
                println!();
            }

            cycles += 1;

            if xpos.abs_diff(x) <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        };
    }
    for line in s.lines() {
        if let Some((_, num)) = line.split_once("addx ") {
            let adder = num.parse::<i64>()?;
            cycle!();
            cycle!();
            x += adder;
        } else {
            cycle!();
        }
    }
    Ok(())
}
