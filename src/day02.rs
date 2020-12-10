use crate::day::Day;
use regex::Regex;

pub struct Day02<'a> {
    passwords: Vec<(usize, usize, char, &'a str)>,
}

impl<'a> Day<'a> for Day02<'a> {
    fn new(input: &'a str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
        Self {
            passwords: input
                .lines()
                .map(|l| re.captures(l).unwrap())
                .map(|c| {
                    Some((
                        c.get(1)?.as_str().parse::<usize>().unwrap(),
                        c.get(2)?.as_str().parse::<usize>().unwrap(),
                        c.get(3)?.as_str().chars().next()?,
                        c.get(4)?.as_str().to_string(),
                    ))
                })
                .map(|r| r.unwrap())
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
                    vec![*a, *b]
                        .into_iter()
                        .filter(|&c| password.as_bytes()[c - 1] == *required_char as u8)
                        .count()
                        == 1
                })
                .count(),
        )
    }
}
