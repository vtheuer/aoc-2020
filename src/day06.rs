use crate::day::Day;

pub struct Day06 {
    groups: Vec<Vec<u32>>,
}

impl Day06 {
    fn sum_groups_with<F: Fn(u32, &u32) -> u32>(&self, neutral: u32, folder: F) -> Box<u32> {
        Box::new(
            self.groups
                .iter()
                .map(|group| group.iter().fold(neutral, &folder).count_ones())
                .sum::<u32>(),
        )
    }
}

impl Day<'_> for Day06 {
    fn new(input: &str) -> Self {
        Day06 {
            groups: input
                .split("\n\n")
                .map(|group| {
                    group
                        .lines()
                        .map(|l| {
                            l.chars()
                                .map(|c| c as u8 - 'a' as u8)
                                .fold(0, |a, c| a | (1 << c))
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        self.sum_groups_with(0, |a, b| a | b)
    }

    fn part_2(&self) -> Box<dyn ToString> {
        self.sum_groups_with((0..26).fold(0, |a, n| a | (1 << n)), |a, b| a & b)
    }
}