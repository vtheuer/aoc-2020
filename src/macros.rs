extern crate proc_macro;

use itertools::Itertools;
use proc_macro::TokenStream;

#[proc_macro]
pub fn run_day(token_stream: TokenStream) -> TokenStream {
    let tokens = token_stream
        .into_iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        tokens.len(),
        2,
        "run_day requires a day count and a day number"
    );
    let day_count = tokens
        .get(0)
        .unwrap()
        .parse::<u8>()
        .expect("day count must be a u8 litteral");
    let day_number = tokens.get(1).unwrap();

    format!(
        "match {0} {{{1}, _ => unreachable!(\"day {{}} not found\", {0})}}",
        day_number,
        (1..=day_count)
            .map(|n| format!(
                "{0} => {{crate::day{0:02}::Day{0:02}::run({0}, &std::fs::read_to_string(\"inputs/{0:02}.txt\").unwrap())}}",
                n
            ))
            .join(","),
    )
    .parse()
    .unwrap()
}
