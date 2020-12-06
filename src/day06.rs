use crate::day::Day;
use std::collections::HashSet;

pub struct Day06 {
    groups: Vec<Vec<HashSet<char>>>,
}

impl Day for Day06 {
    fn new(input: &str) -> Self {
        Day06 {
            groups: input
                .split("\n\n")
                .map(|group| group.lines().map(|l| l.chars().collect()).collect())
                .collect(),
        }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        Box::new(
            self.groups
                .iter()
                .map(|group| {
                    group
                        .iter()
                        .fold(HashSet::new(), |a, b| a.union(b).map(|&c| c).collect())
                        .len()
                })
                .sum::<usize>(),
        )
    }

    fn part_2(&self) -> Box<dyn ToString> {
        Box::new(
            self.groups
                .iter()
                .map(|group| {
                    group
                        .iter()
                        .fold(('a'..='z').collect::<HashSet<_>>(), |a, b| {
                            a.intersection(b).map(|&c| c).collect()
                        })
                        .len()
                })
                .sum::<usize>(),
        )
    }
}