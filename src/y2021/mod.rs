use crate::aoc::Year;

mod d1;
mod d2;
mod d3;

crate::days!(DAYS = d1, d2, d3);

pub const YEAR: Year = Year {
    year: 2021,
    days: DAYS,
};
