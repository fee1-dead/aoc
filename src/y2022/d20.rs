use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut list: Vec<i64> = s
        .lines()
        .map(|x| x.parse().map_err(Into::into))
        .collect::<Result<_>>()?;
    
    for i in 0..list.len() {
        let mut new_pos = list[i] + i as i64;
        while new_pos < 0 {
            new_pos += list.len() as i64;
        }
        new_pos = new_pos % list.len() as i64;
        list.insert(new_pos as usize, list[i]);
        let index = if i < new_pos as usize { i } else { i + 1 };
        list.remove(index);
    }
    dbg!(list[1000] + list[2000] + list[3000]);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    Ok(())
}
