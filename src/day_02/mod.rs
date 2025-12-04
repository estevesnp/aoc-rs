mod part_1;
mod part_2;

use crate::Day;

struct DayImpl;
impl Day for DayImpl {
    fn part_1(&self) -> Result<usize, String> {
        part_1::run()
    }

    fn part_2(&self) -> Result<usize, String> {
        part_2::run()
    }
}

pub fn runner() -> impl Day {
    DayImpl
}
