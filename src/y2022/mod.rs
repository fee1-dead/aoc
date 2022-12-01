use crate::aoc::Year;

mod d01;

crate::days!(DAYS = d01);

pub const YEAR: Year = Year {
    year: 2022,
    days: DAYS,
};
