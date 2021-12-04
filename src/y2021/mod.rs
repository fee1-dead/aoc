use crate::aoc::Year;

mod d1;
mod d2;
mod d3;
mod d4;

crate::days!(DAYS = d1, d2, d3, d4);

pub const YEAR: Year = Year {
    year: 2021,
    days: DAYS,
};
