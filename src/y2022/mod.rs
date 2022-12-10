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

crate::days!(DAYS = d01, d02, d03, d04, d05, d06, d07, d08, d09, d10);

pub const YEAR: Year = Year {
    year: 2022,
    days: DAYS,
};
