use crate::day::Day;

pub struct Day03 {
    grid: Vec<Vec<bool>>,
}

impl Day03 {
    fn count_trees(&self, (right, down): (usize, usize)) -> usize {
        self.grid
            .iter()
            .enumerate()
            .step_by(down)
            .filter(|&(y, row)| row[(y / down * right) % row.len()])
            .count()
    }
}

impl Day for Day03 {
    fn new(input: &str) -> Self {
        Day03 {
            grid: input
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    fn part_1(&self) -> Box<dyn ToString + '_> {
        Box::new(self.count_trees((3, 1)))
    }

    fn part_2(&self) -> Box<dyn ToString> {
        Box::new(
            [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
                .iter()
                .map(|&slope| self.count_trees(slope))
                .product::<usize>(),
        )
    }
}
