use crate::day::Day;
use fnv::FnvHashMap;

pub struct Day15 {
    numbers: Vec<usize>,
}

impl Day15 {
    fn play(&self, end: usize) -> usize {
        let start = self.numbers.len() - 1;
        (start..end - 1)
            .fold(
                (
                    self.numbers
                        .iter()
                        .take(start)
                        .enumerate()
                        .map(|(i, n)| (*n, i))
                        .collect::<FnvHashMap<_, _>>(),
                    self.numbers[start],
                ),
                |(mut last_occurrences, previous), i| {
                    let next = last_occurrences.get(&previous).map(|&j| i - j).unwrap_or(0);
                    last_occurrences.insert(previous, i);

                    (last_occurrences, next)
                },
            )
            .1
    }
}

impl Day<'_> for Day15 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Day15 {
            numbers: input
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.play(2020)
    }

    fn part_2(&self) -> Self::T2 {
        self.play(30_000_000)
    }
}
