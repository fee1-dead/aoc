use crate::aoc::Year;

mod d1;

crate::days!(DAYS = d1);

pub const YEAR: Year = Year {
    year: 2021,
    days: DAYS,
};
