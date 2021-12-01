use crate::aoc::Year;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

crate::days!(DAYS = d1, d2, d3, d4, d5, d6, d7, d8, d9);

pub const YEAR: Year = Year {
    year: 2015,
    days: DAYS,
};
