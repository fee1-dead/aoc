use crate::*;

fn adjacents(n: usize) -> Vec<usize> {
    let mut vec = vec![];

    let x = n % 10;
    let y = n / 10;

    let xm = x > 0;
    let xp = x < 9;
    let ym = y > 0;
    let yp = y < 9;    

    if xm && ym {
        vec.push(x - 1 + (y - 1) * 10);
    }

    if xm && yp {
        vec.push(x - 1 + (y + 1) * 10);
    }

    if xp && ym {
        vec.push(x + 1 + (y - 1) * 10);
    }

    if xp && yp {
        vec.push(x + 1 + (y + 1) * 10);
    }

    if xm {
        vec.push(x - 1 + y * 10);
    }

    if ym {
        vec.push(x + (y - 1) * 10);
    }

    if xp {
        vec.push(x + 1 + y * 10);
    }

    if yp {
        vec.push(x + (y + 1) * 10);
    }

    vec
}

fn simulate(levels: &mut [u8; 100], counter: &mut usize) {
    fn flash(pos: usize, levels: &mut [u8; 100], visited: &mut [bool; 100], counter: &mut usize) {
        *counter += 1;
        visited[pos] = true;
        for n in adjacents(pos) {
            if levels[n] != 10 {
                levels[n] += 1;
                if levels[n] == 10 && !visited[n] {
                    flash(n, levels, visited, counter);
                }
            }
        }
    }
    for l in &mut *levels { *l += 1 }

    let mut visited = [false; 100];
    for n in 0..100 {
        if levels[n] == 10 && !visited[n] {
            flash(n, levels, &mut visited, counter);
        }
    }

    for n in levels {
        if *n == 10 {
            *n = 0;
        }
    }
}

pub fn part1(s: String) -> Result<()> {
    let mut energy_levels = [0u8; 100];

    for (l, chunk) in s.lines().zip(energy_levels.array_chunks_mut::<10>()) {
        for (ch, num) in l.chars().zip(chunk.iter_mut()) {
            assert!(ch.is_ascii_digit());

            *num = ch as u8 - b'0';
        }
    }

    let mut counter = 0;

    for _ in 0..100 {
        simulate(&mut energy_levels, &mut counter);
    }

    dbg!(counter);
    
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut energy_levels = [0u8; 100];

    for (l, chunk) in s.lines().zip(energy_levels.array_chunks_mut::<10>()) {
        for (ch, num) in l.chars().zip(chunk.iter_mut()) {
            assert!(ch.is_ascii_digit());

            *num = ch as u8 - b'0';
        }
    }

    for cnt in 1.. {
        simulate(&mut energy_levels, &mut 0);
        if energy_levels.iter().all(|it| *it == 0) {
            dbg!(cnt);
            break;
        }
    }

    Ok(())
}