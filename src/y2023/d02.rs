use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut sum = 0;
    'x: for (n, l) in s.lines().enumerate() {
        let config = l.split_once(": ").unwrap().1.split(";");
        
        for x in config {
            for color in x.trim().split(", ") {
                let (n, color) = color.split_once(" ").unwrap();
                let n = n.parse::<u32>().unwrap();
                if color == "red" && n > 12 || color == "green" && n > 13 || color == "blue" && n > 14 {
                    continue 'x;
                }
            }

        }
        sum += n + 1;
    }
    dbg!(sum);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut sum = 0;
    'x: for (n, l) in s.lines().enumerate() {
        let config = l.split_once(": ").unwrap().1.split(";");
        
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for x in config {
            for color in x.trim().split(", ") {
                let (n, color) = color.split_once(" ").unwrap();
                let n = n.parse::<u32>().unwrap();
                if color == "red" {
                    red = red.max(n);
                }
                if color == "green" {
                    green = green.max(n);
                }
                if color == "blue" {
                    blue = blue.max(n);
                }
            }

        }
        sum += red * green * blue;
    }
    dbg!(sum);
    Ok(())
}
