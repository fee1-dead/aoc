use crate::*;

const V: Vec<char> = Vec::new();

pub fn part1(s: String) -> Result<()> {
    let mut lines = s.lines();
    let mut stacks = [V; 9];
    for line in lines.by_ref().take(8) {
        for (chunk, stack) in line.as_bytes().chunks(4).zip(&mut stacks) {
            if chunk[0] == b'[' {
                stack.push(chunk[1] as char);
            }
        }
    }
    dbg!(&stacks);
    stacks.iter_mut().for_each(|x| x.reverse());
    for line in lines.skip(2) {
        let howmany: u32;
        let idx1: usize;
        let idx2: usize;
        scanfmt!(line, "move {} from {} to {}", howmany, idx1, idx2);
        for _ in 0..howmany {
            let item = stacks[idx1 - 1].pop();
            stacks[idx2 - 1].push(item.unwrap());
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut lines = s.lines();
    let mut stacks = [V; 9];
    for line in lines.by_ref().take(8) {
        for (chunk, stack) in line.as_bytes().chunks(4).zip(&mut stacks) {
            if chunk[0] == b'[' {
                stack.push(chunk[1] as char);
            }
        }
    }
    dbg!(&stacks);
    stacks.iter_mut().for_each(|x| x.reverse());
    for line in lines.skip(2) {
        let howmany: usize;
        let idx1: usize;
        let idx2: usize;
        scanfmt!(line, "move {} from {} to {}", howmany, idx1, idx2);
        let items = stacks[idx1 - 1].split_off(stacks[idx1 - 1].len() - howmany);
        stacks[idx2 - 1].extend(items);
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
    Ok(())
}
