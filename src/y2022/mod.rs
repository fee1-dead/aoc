use crate::aoc::Year;

mod d01;
mod d02;
mod d03;
mod d04;

crate::days!(DAYS = d01, d02, d03, d04);

pub const YEAR: Year = Year {
    year: 2022,
    days: DAYS,
};
