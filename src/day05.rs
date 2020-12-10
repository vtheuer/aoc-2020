use crate::day::Day;

pub struct Day05 {
    seats: Vec<usize>,
}

impl Day<'_> for Day05 {
    fn new(input: &str) -> Self {
        let mut seats = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'B' | 'R' => '1',
                        'F' | 'L' => '0',
                        _ => unreachable!("unexpected char {}", c),
                    })
                    .collect::<String>()
            })
            .map(|n| usize::from_str_radix(&n, 2).unwrap())
            .collect::<Vec<usize>>();
        seats.sort();
        Day05 { seats }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        Box::new(self.seats.iter().max().unwrap())
    }

    fn part_2(&self) -> Box<dyn ToString> {
        Box::new(
            self.seats
                .windows(2)
                .find(|w| w[1] > w[0] + 1)
                .map(|w| w[1] - 1)
                .unwrap(),
        )
    }
}