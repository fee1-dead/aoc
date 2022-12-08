use std::cell::Cell;
use std::rc::Rc;

use crate::*;

#[derive(Default, Debug, Clone)]
pub struct Dir<'a> {
    parent: String,
    size: usize,
    children: Vec<&'a str>,
}

impl Dir<'_> {
    pub fn size(&self, s: &str, map: &HashMap<String, Dir<'_>>) -> usize {
        self.size + self.children.iter().map(|x| {
            let path =format!("{s}/{x}");
            map[&path].size(&path, map)}).sum::<usize>()
    }
}

pub fn inner(s: String, _part: u8) {
    let lines = s.lines().skip(1);
    let mut current = "/".to_owned();
    let mut ls = false;
    let mut current_dir = Dir::default();
    current_dir.parent = "NOOOOO".into();

    let mut dirs = HashMap::default();

    for l in lines {
        if ls {
            if !l.starts_with("$") {
                let mut parts = l.split_whitespace();
                if l.starts_with("dir") {
                    current_dir.children.push(parts.nth(1).unwrap());
                } else {
                    current_dir.size += parts.next().unwrap().parse::<usize>().unwrap();
                }
            }
        }
        if let Some(x) = l.strip_prefix("$ cd ") {
            if ls {
                ls = false;
                if dirs.insert(current.clone(), current_dir.clone()).is_some() {
                    panic!("{current}");
                }
            }
            if x == ".." {
                current = current_dir.parent.clone();
                current_dir = dirs[&current].clone();
            } else {
                current_dir = Dir::default();
                current_dir.parent = current.clone();
                current = format!("{current}/{x}");
            }
        } else if l == "$ ls" {
            ls = true;
        }
    }
    dirs.insert(current, current_dir);

    dbg!(&dirs);

    let mut total = 0;
    let mut m = usize::MAX;
    let used_space = dirs["/"].size("/", &dirs);
    let current_unused = 70000000 - used_space;
    let needs_free = 30000000 - current_unused;
    for (s, dir) in &dirs {
        let t = dir.size(s, &dirs);
        if t <= 100000 {
            total += t;
        }
        
        if t >= needs_free {
            m = m.min(t);
        }
    }

    dbg!(m);
    dbg!(total);
}

pub fn part1(s: String) -> Result<()> {

    inner(s, 1);
    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    inner(s, 2);
    Ok(())
}
