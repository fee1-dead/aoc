use crate::*;

#[derive(Clone, Copy)]
pub struct Cell {
    visited: bool,
}

pub fn part1(s: String) -> Result<()> {
    let mut grid = vec![Cell { visited: false }; 10000 * 10000];
    let mut head_pos = 1000 * 999;
    let mut tail_pos = 1000 * 999;
    grid[tail_pos].visited = true;

    macro_rules! is_touching {
        () => {
            head_pos == tail_pos
                || head_pos == tail_pos + 1
                || head_pos == tail_pos - 1
                || head_pos == tail_pos + 1000
                || head_pos == tail_pos - 1000
                || head_pos == tail_pos + 1001
                || head_pos == tail_pos - 1001
                || head_pos == tail_pos + 999
                || head_pos == tail_pos - 999
        };
    }

    for line in s.lines() {
        let ins: String;
        let n: usize;
        scanfmt!(line, "{} {}", ins, n);
        match &*ins {
            "U" => {
                for _ in 0..n {
                    let prev = head_pos;
                    head_pos -= 1000;
                    if !is_touching!() {
                        tail_pos = prev;
                        grid[tail_pos].visited = true;
                    }
                }
            }
            "D" => {
                for _ in 0..n {
                    let prev = head_pos;
                    head_pos += 1000;
                    if !is_touching!() {
                        tail_pos = prev;
                        grid[tail_pos].visited = true;
                    }
                }
            }
            "L" => {
                for _ in 0..n {
                    let prev = head_pos;
                    head_pos -= 1;
                    if !is_touching!() {
                        tail_pos = prev;
                        grid[tail_pos].visited = true;
                    }
                }
            }
            "R" => {
                for _ in 0..n {
                    let prev = head_pos;
                    head_pos += 1;
                    if !is_touching!() {
                        tail_pos = prev;
                        grid[tail_pos].visited = true;
                    }
                }
            }
            _ => {}
        }
    }

    dbg!(grid.iter().filter(|x| x.visited).count());
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut grid = vec![Cell { visited: false }; 10000 * 10000];
    let mut knots = vec![1000 * 999; 10];
    grid[*knots.last().unwrap()].visited = true;

    macro_rules! is_touching {
        ($prev: expr, $cur: expr) => {
            $prev == $cur
                || $prev == $cur + 1
                || $prev == $cur - 1
                || $prev == $cur + 1000
                || $prev == $cur - 1000
                || $prev == $cur + 1001
                || $prev == $cur - 1001
                || $prev == $cur + 999
                || $prev == $cur - 999
        };
    }

    macro_rules! try_touch {
        ($prev: ident, $cur: ident) => {
            if $prev == $cur - 2 {
                $cur - 1
            } else if $prev == $cur + 2 {
                $cur + 1
            } else if $prev == $cur - 2000 {
                $cur - 1000
            } else if $prev == $cur + 2000 {
                $cur + 1000
            } else if $prev == $cur - 2000 - 2
                || $prev == $cur - 2000 - 1
                || $prev == $cur - 1000 - 2
            {
                $cur - 1000 - 1
            } else if $prev == $cur - 2000 + 2
                || $prev == $cur - 2000 + 1
                || $prev == $cur - 1000 + 2
            {
                $cur - 1000 + 1
            } else if $prev == $cur + 2000 + 2
                || $prev == $cur + 2000 + 1
                || $prev == $cur + 1000 + 2
            {
                $cur + 1000 + 1
            } else if $prev == $cur + 2000 - 2
                || $prev == $cur + 2000 - 1
                || $prev == $cur + 1000 - 2
            {
                $cur + 1000 - 1
            } else {
                unreachable!("{},{}", $prev, $cur)
            }
        };
    }

    for line in s.lines() {
        let ins: String;
        let n: usize;
        scanfmt!(line, "{} {}", ins, n);
        match &*ins {
            "U" => {
                for _ in 0..n {
                    knots[0] -= 1000;

                    for n in 1..10 {
                        let prev_pos = knots[n - 1];
                        let cur_pos = knots[n];
                        if !is_touching!(prev_pos, cur_pos) {
                            knots[n] = try_touch!(prev_pos, cur_pos);
                        }
                    }
                    grid[knots[9]].visited = true;
                }
            }
            "D" => {
                for _ in 0..n {
                    knots[0] += 1000;
                    for n in 1..10 {
                        let prev_pos = knots[n - 1];
                        let cur_pos = knots[n];
                        if !is_touching!(prev_pos, cur_pos) {
                            knots[n] = try_touch!(prev_pos, cur_pos);
                        }
                    }
                    grid[knots[9]].visited = true;
                }
            }
            "L" => {
                for _ in 0..n {
                    knots[0] -= 1;
                    for n in 1..10 {
                        let prev_pos = knots[n - 1];
                        let cur_pos = knots[n];
                        if !is_touching!(prev_pos, cur_pos) {
                            knots[n] = try_touch!(prev_pos, cur_pos);
                        }
                    }
                    grid[knots[9]].visited = true;
                }
            }
            "R" => {
                for _ in 0..n {
                    knots[0] += 1;
                    for n in 1..10 {
                        let prev_pos = knots[n - 1];
                        let cur_pos = knots[n];
                        if !is_touching!(prev_pos, cur_pos) {
                            knots[n] = try_touch!(prev_pos, cur_pos);
                        }
                    }
                    grid[knots[9]].visited = true;
                }
            }
            _ => {}
        }
    }

    dbg!(grid.iter().filter(|x| x.visited).count());

    Ok(())
}
