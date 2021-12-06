use crate::*;

// a(n) = a(n - 7) + a(n - 9)
// http://oeis.org/A122521 but because 0 is valid so the constants should be modified
const TABLE: [u64; 300] = {
    let mut table = [0; 300];
    let mut n = 0;
    while n < 300 {
        if n <= 9 {
            table[n] = 1;
        } else {
            table[n] = table[n - 7] + table[n - 9];
        }
        n += 1;
    }
    table
};

fn solve(s: String, days: usize) -> Result<()> {
    let solution: u64 = s
        .split(',')
        .map(|s| s.parse::<usize>())
        .map(Result::unwrap)
        .map(|initial| TABLE[days - initial + 9])
        .sum();
    dbg!(solution);

    Ok(())
}

pub fn part1(s: String) -> Result<()> {
    solve(s, 80)
}

pub fn part2(s: String) -> Result<()> {
    solve(s, 256)
}
