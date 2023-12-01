use aoc_1::aoc_1;

mod aoc_1;
mod util;

fn main() -> std::io::Result<()> {
    assert!(aoc_1()? == 54019);
    Ok(())
}
