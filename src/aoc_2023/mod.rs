use crate::test_solution;

pub mod day_01;

pub fn bench() {
    test_solution("Day 1 Part 1", day_01::part_one);
    test_solution("Day 1 Part 2", day_01::part_two);
}
