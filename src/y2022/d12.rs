use crate::*;
use std::collections::VecDeque;

pub fn part1(s: String) -> Result<()> {
    let mut grid = [[' '; 80]; 41];
    for line in s.lines().zip(&mut grid) {
        for (c, g) in line.0.chars().zip(line.1.iter_mut()) {
            *g = c;
        }
    }
    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    let end_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'E' { Some((x, y)) } else { None })
        })
        .unwrap();

    let mut visited = [[false; 80]; 41];
    let mut queue = VecDeque::new();
    queue.push_back((start_pos, 0));

    loop {
        macro_rules! map {
            ($x:expr) => {{
                match $x {
                    'S' => 'a',
                    'E' => 'z',
                    x => x,
                }
            }};
        }
        let ((x, y), steps) = queue.pop_front().unwrap();

        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        if (x, y) == end_pos {
            dbg!(steps);
            break;
        }
        let cur = map!(grid[y][x]);

        if x > 0 && (map!(grid[y][x - 1]) as u32 <= cur as u32 + 1) {
            queue.push_back(((x - 1, y), steps + 1));
        }
        if x < 79 && (map!(grid[y][x + 1]) as u32 <= cur as u32 + 1) {
            queue.push_back(((x + 1, y), steps + 1));
        }
        if y > 0 && (map!(grid[y - 1][x]) as u32 <= cur as u32 + 1) {
            queue.push_back(((x, y - 1), steps + 1));
        }
        if y < 40 && (map!(grid[y + 1][x]) as u32 <= cur as u32 + 1) {
            queue.push_back(((x, y + 1), steps + 1));
        }
    }

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut grid = [[' '; 80]; 41];
    for line in s.lines().zip(&mut grid) {
        for (c, g) in line.0.chars().zip(line.1.iter_mut()) {
            *g = c;
        }
    }
    let start_pos = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if *c == 'S' || *c == 'a' {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect::<Vec<_>>();

    let end_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'E' { Some((x, y)) } else { None })
        })
        .unwrap();

    let mut distance = u64::MAX;

    for pos in start_pos {
        let mut visited = [[false; 80]; 41];
        let mut queue = VecDeque::new();
        queue.push_back((pos, 0));

        loop {
            macro_rules! map {
                ($x:expr) => {{
                    match $x {
                        'S' => 'a',
                        'E' => 'z',
                        x => x,
                    }
                }};
            }
            let Some(((x, y), steps)) = queue.pop_front() else { break };

            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;

            if (x, y) == end_pos {
                distance = distance.min(steps);
                break;
            }
            let cur = map!(grid[y][x]);

            if x > 0 && (map!(grid[y][x - 1]) as u32 <= cur as u32 + 1) {
                queue.push_back(((x - 1, y), steps + 1));
            }
            if x < 79 && (map!(grid[y][x + 1]) as u32 <= cur as u32 + 1) {
                queue.push_back(((x + 1, y), steps + 1));
            }
            if y > 0 && (map!(grid[y - 1][x]) as u32 <= cur as u32 + 1) {
                queue.push_back(((x, y - 1), steps + 1));
            }
            if y < 40 && (map!(grid[y + 1][x]) as u32 <= cur as u32 + 1) {
                queue.push_back(((x, y + 1), steps + 1));
            }
        }
    }

    dbg!(distance);

    Ok(())
}
