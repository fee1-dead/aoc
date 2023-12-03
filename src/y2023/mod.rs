use crate::aoc::Year;

mod d01;
mod d02;
mod d03;

crate::days! {
    DAYS = d01,
    d02,
    d03,
}

pub const YEAR: Year = Year {
    year: 2023,
    days: DAYS,
};
