use crate::day::Day;

pub struct Day05 {
    seats: Vec<usize>,
}

impl Day<'_> for Day05 {
    fn new(input: &str) -> Self {
        let mut seats = input
            .lines()
            .map(|l| {
                l.bytes().fold(0, |n, c| {
                    (n << 1)
                        | match c {
                            b'B' | b'R' => 1,
                            b'F' | b'L' => 0,
                            _ => unreachable!("unexpected char {}", c),
                        }
                })
            })
            .collect::<Vec<_>>();
        seats.sort();
        Day05 { seats }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        Box::new(self.seats.last().unwrap())
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
