use crate::day::Day;
use std::cell::Cell;

pub struct Day09 {
    numbers: Vec<usize>,
    invalid: Cell<usize>,
}

impl Day<'_> for Day09 {
    fn new(input: &str) -> Self {
        Day09 {
            numbers: input.lines().map(|l| l.parse().unwrap()).collect(),
            invalid: Cell::new(0),
        }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        let invalid = *self
            .numbers
            .iter()
            .enumerate()
            .skip(25)
            .find(|&(i, n)| {
                let previous = &self.numbers[i - 25..i];
                previous
                    .iter()
                    .enumerate()
                    .flat_map(|(i, a)| previous.iter().skip(i + 1).map(move |b| *a + *b))
                    .find(|m| *m == *n)
                    .is_none()
            })
            .unwrap()
            .1;
        self.invalid.set(invalid);
        Box::new(invalid)
    }

    fn part_2(&self) -> Box<dyn ToString> {
        let invalid = self.invalid.get();

        Box::new(
            self.numbers
                .iter()
                .enumerate()
                .flat_map(|(i, &a)| {
                    self.numbers[i + 1..]
                        .iter()
                        .scan(a, |sum, &b| {
                            *sum += b;
                            Some(*sum)
                        })
                        .take_while(|n| *n <= invalid)
                        .enumerate()
                        .map(move |(j, sum)| (i, i + j, sum))
                })
                .find(|(_, _, sum)| *sum == invalid)
                .map(|(i, j, _)| {
                    self.numbers[i..=j]
                        .iter()
                        .fold((usize::MAX, 0), |(min, max), &n| (min.min(n), max.max(n)))
                })
                .map(|(min, max)| min + max)
                .unwrap(),
        )
    }
}
