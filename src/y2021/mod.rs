use crate::aoc::Year;

mod d1;
mod d2;

crate::days!(DAYS = d1, d2);

pub const YEAR: Year = Year {
    year: 2021,
    days: DAYS,
};
