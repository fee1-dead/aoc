use std::collections::hash_map::Entry;

use crate::*;

pub fn part1(s: String) -> Result<()> {
    let mut nice = 0;

    'outer: for l in s.lines() {
        let mut vowels = 0;
        let mut double_letters = false;
        let mut last_char = ' ';

        for c in l.chars() {
            if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
                vowels += 1;
            }

            match (last_char, c) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => continue 'outer,
                _ => {}
            }

            if c == last_char {
                double_letters = true;
            }

            last_char = c;
        }

        if vowels >= 3 && double_letters {
            nice += 1;
        }
    }

    dbg!(nice);

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut nice = 0;
    let mut pairs = HashMap::default();

    for l in s.lines() {
        pairs.clear();
        let mut repeat = false;
        let mut repeated_pair = false;
        let mut last: &[_] = &[b' ', b' '];

        for (n, current) in l.as_bytes().windows(2).enumerate() {
            if last[0] == current[1] {
                repeat = true;
            }

            match pairs.entry(current) {
                Entry::Occupied(o) => {
                    // must differ by two chars or else they overlap
                    if n - *o.get() >= 2 {
                        repeated_pair = true;
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(n);
                }
            }

            if repeat && repeated_pair {
                nice += 1;
                break;
            }

            last = current;
        }
    }

    dbg!(nice);

    Ok(())
}
