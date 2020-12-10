pub fn split_pair<'a>(input: &'a str, p: &str) -> Option<(&'a str, &'a str)> {
    let mut s = input.splitn(2, p);
    Some((s.next()?, s.next()?))
}

pub fn rsplit_pair<'a>(input: &'a str, p: &str) -> Option<(&'a str, &'a str)> {
    let mut s = input.rsplitn(2, p);
    let (a, b) = (s.next()?, s.next()?);
    Some((b, a))
}
