use crate::*;

#[derive(Default, Clone, Copy)]
pub struct Cell {
    x: i32,
    visible: bool,
}

pub fn part1(s: String) -> Result<()> {
    let size = s.lines().next().unwrap().len();

    let mut grid = vec![Cell::default(); size * size];

    for (ch, c) in s.lines().flat_map(|x| x.chars()).zip(grid.iter_mut()) {
        c.x = ch.to_digit(10).unwrap() as i32;
    }

    // horizontal sweep
    for row in grid.chunks_mut(size) {
        let mut max = -1;

        for c in row.iter_mut() {
            if c.x > max {
                c.visible = true;
            }
            max = max.max(c.x);
        }

        max = -1;
        for c in row.iter_mut().rev() {
            if c.x > max {
                c.visible = true;
            }
            max = max.max(c.x);
        }
    }

    // vertical
    for col in 0..size {
        let mut max = -1;
        for row in grid.iter_mut().skip(col).step_by(size) {
            if row.x > max {
                row.visible = true;
            }
            max = max.max(row.x);
        }
        max = -1;
        for row in grid.iter_mut().skip(col).step_by(size).rev() {
            if row.x > max {
                row.visible = true;
            }
            max = max.max(row.x);
        }
    }

    dbg!(grid.iter().filter(|x| x.visible).count());

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let size = s.lines().next().unwrap().len();

    let mut grid = vec![Cell::default(); size * size];

    for (ch, c) in s.lines().flat_map(|x| x.chars()).zip(grid.iter_mut()) {
        c.x = ch.to_digit(10).unwrap() as i32;
    }

    let mut max_score = 0;
    for (pos, cell) in grid.iter().enumerate() {
        let height = cell.x;
        let mut score = 1;
        
        let x = pos % size;
        let y = pos / size;
        // up
        let mut count = 0;
        for y in (0..y).rev() {
            let c = &grid[y * size + x];
            if c.x < height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }
        score *= count;
        // down
        let mut count = 0;
        for y in (y + 1)..size {
            let c = &grid[y * size + x];
            if c.x < height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }
        score *= count;
        // left
        let mut count = 0;
        for x in (0..x).rev() {
            let c = &grid[y * size + x];
            if c.x < height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }
        score *= count;
        // right
        let mut count = 0;
        for x in (x + 1)..size {
            let c = &grid[y * size + x];
            if c.x < height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }
        score *= count;
        max_score = max_score.max(score);
    }

    dbg!(max_score);

    Ok(())
}
