use crate::*;

impl_day!(DayOne);

impl Solve for DayOne {
    fn solve(&self) -> String {
        self.file_contents.to_owned()
    }
}
