use crate::day::Day;

pub struct Day01 {
    numbers: Vec<u32>,
}

impl Day<'_> for Day01 {
    fn new(input: &str) -> Self {
        Day01 {
            numbers: input
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        Box::new(
            self.numbers
                .iter()
                .enumerate()
                .flat_map(|(i, &a)| self.numbers.iter().skip(i + 1).map(move |&b| (a, b)))
                .into_iter()
                .find(|(a, b)| a + b == 2020)
                .map(|(a, b)| a * b)
                .unwrap(),
        )
    }

    fn part_2(&self) -> Box<dyn ToString> {
        Box::new(
            self.numbers
                .iter()
                .enumerate()
                .flat_map(|(i, &a)| {
                    self.numbers
                        .iter()
                        .enumerate()
                        .skip(i + 1)
                        .filter(move |(_, &b)| a + b < 2020)
                        .flat_map(|(j, &b)| self.numbers.iter().skip(j + 1).map(move |&c| (b, c)))
                        .map(move |(b, c)| (a, b, c))
                })
                .into_iter()
                .find(|&(a, b, c)| a + b + c == 2020)
                .map(|(a, b, c)| a * b * c)
                .unwrap(),
        )
    }
}
