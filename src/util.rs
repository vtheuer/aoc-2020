pub fn split_pair<'a>(input: &'a str, p: &str) -> Option<(&'a str, &'a str)> {
    let mut s = input.splitn(2, p);
    Some((s.next()?, s.next()?))
}

pub fn rsplit_pair<'a>(input: &'a str, p: &str) -> Option<(&'a str, &'a str)> {
    let mut s = input.rsplitn(2, p);
    let (a, b) = (s.next()?, s.next()?);
    Some((b, a))
}

pub fn format_duration(time: u128) -> String {
    let ftime = time as f64;
    if ftime <= 1e3 {
        format!("{:.0}ns", ftime)
    } else if ftime <= 1e6 {
        format!("{:.1}µs", ftime / 1e3)
    } else if ftime <= 1e9 {
        format!("{:.1}ms", ftime / 1e6)
    } else {
        format!("{:.1} s", ftime / 1e9)
    }
}
