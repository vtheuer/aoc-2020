use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::day::Day;
use crate::util::{rsplit_pair, split_pair};

pub struct Day07<'a> {
    rules: HashMap<&'a str, HashMap<&'a str, u32>>,
}

impl Day07<'_> {
    fn count_bags_in(&self, container: &str) -> u32 {
        self.rules
            .get(container)
            .unwrap()
            .into_iter()
            .map(|(bag, count)| *count * (1 + self.count_bags_in(bag)))
            .sum()
    }
}

fn bags_containing<'a>(
    containers_by_bag: &HashMap<&str, Vec<&'a str>>,
    bag: &str,
) -> HashSet<&'a str> {
    containers_by_bag
        .get(bag)
        .map(|bags| {
            bags.into_iter()
                .fold(HashSet::new(), |mut containers, container| {
                    containers.insert(*container);
                    containers.extend(bags_containing(containers_by_bag, container));
                    containers
                })
        })
        .unwrap_or_else(HashSet::new)
}

impl<'a> Day<'a> for Day07<'a> {
    fn new(input: &'a str) -> Self {
        Day07::<'a> {
            rules: input
                .lines()
                .map(|l| {
                    let (bag, content) = split_pair(l, " bags contain ")?;
                    Some((
                        bag,
                        content
                            .split(", ")
                            .filter_map(|content| {
                                let (count_and_bag, _) = rsplit_pair(content, " ")?;
                                let (count, bag) = split_pair(count_and_bag, " ")?;

                                Some((bag, count.parse().ok()?))
                            })
                            .collect(),
                    ))
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        Box::new(
            bags_containing(
                &self
                    .rules
                    .iter()
                    .flat_map(|(container, content)| {
                        content.into_iter().map(move |(bag, _)| (*bag, *container))
                    })
                    .into_group_map(),
                "shiny gold",
            )
            .len(),
        )
    }

    fn part_2(&self) -> Box<dyn ToString> {
        Box::new(self.count_bags_in("shiny gold"))
    }
}
