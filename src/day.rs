use std::time::Instant;

fn time<T, F: Fn() -> T>(f: F) -> (T, f64) {
    let begin = Instant::now();
    (f(), begin.elapsed().as_secs_f64() * 1e3)
}

pub trait Day: Sized {
    fn new(input: &str) -> Self;
    fn part_1(&self) -> Box<dyn ToString>;
    fn part_2(&self) -> Box<dyn ToString>;

    fn run_part(&self, part: fn(&Self) -> Box<dyn ToString>) -> String {
        let (output, duration) = time(|| part(self));
        format!(
            "{}\nRun time: {:.3}ms\n",
            output.as_ref().to_string(),
            duration
        )
    }

    fn run(input: &str) {
        let (day, parse_duration) = time(|| Self::new(input));
        println!("Parse time: {:.3}ms", parse_duration);

        print!("\nPart 1: {}", day.run_part(Day::part_1));
        print!("\nPart 2: {}", day.run_part(Day::part_2));
    }
}
