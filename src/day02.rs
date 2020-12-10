use crate::day::Day;
use crate::util::split_pair;

pub struct Day02<'a> {
    passwords: Vec<(usize, usize, char, &'a str)>,
}

impl<'a> Day<'a> for Day02<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            passwords: input
                .lines()
                .map(|l| {
                    let (rule, password) = split_pair(l, ": ")?;
                    let (range, c) = split_pair(rule, " ")?;
                    let (min, max) = split_pair(range, "-")?;

                    Some((
                        min.parse().ok()?,
                        max.parse().ok()?,
                        c.chars().nth(0)?,
                        password,
                    ))
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        Box::new(
            self.passwords
                .iter()
                .filter(|(min, max, required_char, password)| {
                    let count = password.chars().filter(|&c| c == *required_char).count();
                    count >= *min && count <= *max
                })
                .count(),
        )
    }

    fn part_2(&self) -> Box<dyn ToString> {
        Box::new(
            self.passwords
                .iter()
                .filter(|(a, b, required_char, password)| {
                    [*a, *b]
                        .iter()
                        .filter(|&c| password.as_bytes()[c - 1] == *required_char as u8)
                        .count()
                        == 1
                })
                .count(),
        )
    }
}
