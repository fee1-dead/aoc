use crate::aoc::Year;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;

crate::days!(
    DAYS = d01,
    d02,
    d03,
    d04,
    d05,
    d06,
    d07,
    d08,
    d09,
    d10,
    d11,
    d12,
    d13,
    d14,
    d15,
    d16,
    d17,
    d18,
    d19,
    d20,
);

pub const YEAR: Year = Year {
    year: 2022,
    days: DAYS,
};
