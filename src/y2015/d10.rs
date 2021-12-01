use crate::*;

pub fn look_and_say(s: &str) -> String {
    let mut last_char = s.chars().next().unwrap();
    let mut last_count = b'0';
    let mut st = String::new();

    for ch in s.chars() {
        if last_char != ch {
            st.push(last_count as char);
            st.push(last_char);

            last_char = ch;
            last_count = b'1';
        } else {
            last_count += 1;
        }
    }

    st.push(last_count as char);
    st.push(last_char);

    st
}

pub fn part1(mut s: String) -> Result<()> {
    for _ in 0..40 {
        s = look_and_say(&s);
    }

    dbg!(s.len());

    Ok(())
}

pub fn part2(mut s: String) -> Result<()> {
    for _ in 0..50 {
        s = look_and_say(&s);
    }

    dbg!(s.len());

    Ok(())
}