use std::fs;
use std::path::Path;
use std::time::Instant;
use std::{env::args, fs::File, io::Read};

use reqwest::blocking::*;
use reqwest::header::HeaderMap;
use reqwest::header::COOKIE;

use aoc::{bail, Context, Result, YEARS};

fn url(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

const SESSION_COOKIE: &str = concat!("session=", include_str!("../SESSION"));

fn get(year: u16, day: u8) -> Result<String> {
    let file = format!("./input/{year}-{day:02}.txt");

    if Path::new(&file).exists() {
        Ok(fs::read_to_string(file)?)
    } else {
        let mut headers = HeaderMap::new();
        headers.append(COOKIE, SESSION_COOKIE.parse()?);
        let client = Client::builder().default_headers(headers).build()?;

        let res = client.execute(client.get(url(year, day)).build()?)?;
        let text = res.text()?;
        fs::write(file, &text)?;

        Ok(text)
    }
}

fn main() -> Result<()> {
    let mut args = args().skip(1);

    let year = args
        .next()
        .with_context(|| "expected YYYY argument")?
        .parse::<u16>()?;
    let day = args
        .next()
        .with_context(|| "expected day argument")?
        .parse::<u8>()?;
    let part = args
        .next()
        .with_context(|| "expected part")?
        .parse::<u8>()?;

    for y in YEARS {
        if y.year == year {
            let d = y
                .days
                .get(day as usize - 1)
                .with_context(|| "day does not exist")?;
            let runner = match part {
                1 => d.part1,
                2 => d.part2,
                _ => bail!("invalid part"),
            };

            let text = get(year, day)?;
            println!("Got text");
            let ins = Instant::now();
            runner(text)?;
            println!("Completed in {:?}", ins.elapsed());

            return Ok(());
        }
    }

    bail!("year not found")
}
