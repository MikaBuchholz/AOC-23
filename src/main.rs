use aoc_1::aoc_1;
use aoc_2::aoc_2;
use aoc_3::{aoc_3_part_1, aoc_3_part_2};

mod aoc_1;
mod aoc_2;
mod aoc_3;
mod util;

fn main() -> std::io::Result<()> {
    //assert!(aoc_1()? == 54019);
    //assert!(aoc_2()? == 62241);
    //assert!(aoc_3_part_1()? == 528819);
    aoc_3_part_2()?;
    Ok(())
}
