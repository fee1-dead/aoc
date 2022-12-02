use crate::aoc::Year;

mod d01;
mod d02;

crate::days!(DAYS = d01, d02);

pub const YEAR: Year = Year {
    year: 2022,
    days: DAYS,
};
