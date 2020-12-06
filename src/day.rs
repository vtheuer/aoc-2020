use std::time::Instant;

fn time<T, F: Fn() -> T>(f: F) -> (T, f64) {
    let begin = Instant::now();
    (f(), begin.elapsed().as_secs_f64() * 1e3)
}

pub trait Day: Sized {
    fn new(input: &str) -> Self;
    fn part_1(&self) -> Box<dyn ToString + '_>;
    fn part_2(&self) -> Box<dyn ToString>;

    fn run_part(&self, part: fn(&Self) -> Box<dyn ToString + '_>) -> (String, f64) {
        let (output, duration) = time(|| part(self));
        (
            format!(
                "{}\nRun time: {:.3}ms\n",
                output.as_ref().to_string(),
                duration
            ),
            duration,
        )
    }

    fn run(input: &str) -> f64 {
        let (day, parse_duration) = time(|| Self::new(input));
        println!("Parse time: {:.3}ms", parse_duration);

        let (output_1, part_1_duration) = day.run_part(Day::part_1);
        print!("Part 1: {}", output_1);
        let (output_2, part_2_duration) = day.run_part(Day::part_2);
        print!("Part 2: {}", output_2);

        parse_duration + part_1_duration + part_2_duration
    }
}