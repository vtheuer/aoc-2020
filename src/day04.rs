use crate::day::Day;
use regex::Regex;

pub struct Day04 {
    passports: Vec<Vec<(String, String)>>,
}

impl Day for Day04 {
    fn new(input: &str) -> Self {
        Day04 {
            passports: input
                .split("\n\n")
                .map(|passport| {
                    passport
                        .split(|c| c == ' ' || c == '\n')
                        .filter(|pair| !pair.is_empty())
                        .map(|pair| {
                            let mut split = pair.split(':').map(String::from);
                            (split.next().unwrap(), split.next().unwrap())
                        })
                        .collect::<Vec<_>>()
                })
                .filter(|pairs| {
                    pairs.len()
                        >= pairs
                            .iter()
                            .find(|(k, _)| k == "cid")
                            .and(Some(8))
                            .unwrap_or(7)
                })
                .collect::<Vec<_>>(),
        }
    }
    fn part_1(&self) -> Box<dyn ToString> {
        Box::new(self.passports.len())
    }

    fn part_2(&self) -> Box<dyn ToString> {
        let height_pattern = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        let hair_color_pattern = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let eye_color_pattern = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        let pid_pattern = Regex::new(r"^\d{9}$").unwrap();
        Box::new(
            self.passports
                .iter()
                .filter(|pairs| {
                    pairs.iter().all(|(k, v)| match k.as_str() {
                        "byr" => v
                            .parse::<u32>()
                            .map(|y| y >= 1920 && y <= 2002)
                            .unwrap_or(false),
                        "iyr" => v
                            .parse::<u32>()
                            .map(|y| y >= 2010 && y <= 2020)
                            .unwrap_or(false),
                        "eyr" => v
                            .parse::<u32>()
                            .map(|y| y >= 2020 && y <= 2030)
                            .unwrap_or(false),
                        "hgt" => height_pattern
                            .captures(v)
                            .and_then(|c| Some((c.get(1)?, c.get(2)?)))
                            .and_then(|(h, u)| Some((h.as_str().parse::<u32>().ok()?, u.as_str())))
                            .filter(|&(h, u)| {
                                if u == "cm" {
                                    h >= 150 && h <= 193
                                } else {
                                    h >= 59 && h <= 76
                                }
                            })
                            .is_some(),
                        "hcl" => hair_color_pattern.is_match(v),
                        "ecl" => eye_color_pattern.is_match(v),
                        "pid" => pid_pattern.is_match(v),
                        "cid" => true,
                        _ => panic!("unknown key {}", k),
                    })
                })
                .count(),
        )
    }
}
