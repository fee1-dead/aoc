#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(decl_macro)]
#![feature(iter_advance_by)]

pub use color_eyre::eyre::*;
pub use color_eyre::{Report, Result};
pub use fxhash::FxHashMap as HashMap;
pub use fxhash::FxHashSet as HashSet;
pub use itertools::Itertools;
pub use scanfmt::scanfmt;
pub use std::convert::identity;
pub use tap::Pipe;

pub mod aoc;

mod y2015;
mod y2020;
mod y2021;
mod y2022;
mod y2023;

pub macro years($years:ident = $($module:ident),+) {
    pub const $years: &[$crate::aoc::Year] = &[
        $($module::YEAR),+
    ];
}

pub macro days($days:ident = $($day:ident),+$(,)?) {
    pub const $days: &[$crate::aoc::Day] = &[
        $(
            $crate::aoc::Day {
                part1: $day::part1,
                part2: $day::part2,
            }
        ),+
    ];
}

years!(YEARS = y2015, y2020, y2021, y2022, y2023);
