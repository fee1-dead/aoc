use crate::*;

fn low_points(grid: &[u8], width: usize) -> impl Iterator<Item = (usize, &u8)> + '_ {
    let height = grid.len() / width;
    grid.iter().enumerate().filter(move |&(pos, &digit)| {
        let x = pos % width;
        let y = pos / width;

        let xplus1 = x + 1 < width && digit >= grid[y * width + x + 1];
        let xminus1 = x > 0 && digit >= grid[y * width + x - 1];
        let yplus1 = y + 1 < height && digit >= grid[(y + 1) * width + x];
        let yminus1 = y > 0 && digit >= grid[(y - 1) * width + x];

        !xplus1 && !xminus1 && !yplus1 && !yminus1
    })
}

pub fn part1(s: String) -> Result<()> {
    let grid = s
        .lines()
        .flat_map(|s| s.chars().map(|c| c as u8 - b'0'))
        .collect_vec();
    let width = s.lines().next().unwrap().len();

    dbg!(low_points(&grid, width)
        .map(|(_, &digit)| digit as usize + 1)
        .sum::<usize>());

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let grid = s
        .lines()
        .flat_map(|s| s.chars().map(|c| c as u8 - b'0'))
        .collect_vec();
    let width = s.lines().next().unwrap().len();
    let height = grid.len() / width;

    let ans = low_points(&grid, width)
        .map(|(pos, _)| {
            let x = pos % width;
            let y = pos / width;
            let mut visited: HashSet<(usize, usize)> = HashSet::default();

            fn visit(
                x: usize,
                y: usize,
                visited: &mut HashSet<(usize, usize)>,
                grid: &[u8],
                width: usize,
                height: usize,
            ) {
                if grid[x + y * width] == 9 {
                    return;
                }
                if !visited.insert((x, y)) {
                    return;
                }

                if x + 1 < width {
                    visit(x + 1, y, visited, grid, width, height);
                }
                if x > 0 {
                    visit(x - 1, y, visited, grid, width, height);
                }
                if y + 1 < height {
                    visit(x, y + 1, visited, grid, width, height);
                }
                if y > 0 {
                    visit(x, y - 1, visited, grid, width, height);
                }
            }

            visit(x, y, &mut visited, &grid, width, height);

            visited.len()
        })
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<usize>();

    dbg!(ans);

    Ok(())
}
