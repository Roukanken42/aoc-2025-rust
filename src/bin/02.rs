use advent_of_code::utils::{Parsable, parse_input};
use itertools::Itertools;
use nom::character::char;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use std::ops::Div;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct Interval {
    start: u64,
    end: u64,
}

impl Interval {
    pub fn new(start: u64, end: u64) -> Interval {
        Interval { start, end }
    }
}

impl Parsable<'_> for Interval {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (start, end)) =
            separated_pair(u64::parse, char('-'), u64::parse).parse(input)?;

        Ok((input, Interval::new(start, end)))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Interval>> {
    parse_input(separated_list0(char(','), Interval::parse)).parse(input)
}

fn double(number: u64) -> u64 {
    let length = number.ilog10() + 1;
    10u64.pow(length) * number + number
}

fn halve(number: u64) -> u64 {
    let length = number.ilog10() + 1;
    let first_half = number.div(10u64.pow(length / 2));

    if length % 2 == 1 {
        10u64.pow(length / 2)
    } else if double(first_half) >= number {
        first_half
    } else {
        first_half + 1
    }
}

fn repeat(base: u64, times: u32) -> u64 {
    let length = base.ilog10() + 1;
    let mut result = base;

    for _ in 0..times - 1 {
        result = result * 10u64.pow(length) + base;
    }

    result
}

fn split(number: u64, parts: u32) -> u64 {
    let length = number.ilog10() + 1;
    let part_length = (length - 1) / parts + 1;
    let base = number.div(10u64.pow(length - part_length));

    if length % parts != 0 {
        10u64.pow(length / parts)
    } else if repeat(base, parts) >= number {
        base
    } else {
        base + 1
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, data) = parse(input).unwrap();

    let result = data
        .into_iter()
        .map(|interval| {
            (halve(interval.start)..halve(interval.end + 1))
                .map(double)
                .sum::<u64>() // possible speedup with summing up same-digit numbers
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, data) = parse(input).unwrap();

    fn all_invalid_bases_with_parts(interval: &Interval, parts: u32) -> impl Iterator<Item = u64> {
        (split(interval.start, parts)..split(interval.end + 1, parts))
            .map(move |base| repeat(base, parts))
    }

    let result = data
        .iter()
        .flat_map(|interval| {
            let max_parts = interval.end.ilog10() + 1;
            (2..=max_parts).map(move |parts| (interval, parts))
        })
        .flat_map(|(interval, parts)| all_invalid_bases_with_parts(interval, parts))
        .unique()
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse(&input);

        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Interval::new(11, 22),
                    Interval::new(95, 115),
                    Interval::new(998, 1012),
                    Interval::new(1188511880, 1188511890),
                    Interval::new(222220, 222224),
                    Interval::new(1698522, 1698528),
                    Interval::new(446443, 446449),
                    Interval::new(38593856, 38593862),
                    Interval::new(565653, 565659),
                    Interval::new(824824821, 824824827),
                    Interval::new(2121212118, 2121212124),
                ]
            ))
        );
    }

    #[test]
    fn test_nearest_invalid_base_upwards() {
        assert_eq!(halve(11), 1);
        assert_eq!(halve(95), 9);
        assert_eq!(halve(998), 10);
        assert_eq!(halve(115), 10);
        assert_eq!(halve(11515), 100);
        assert_eq!(halve(1188511880), 11885);
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat(12, 3), 121212);
        assert_eq!(repeat(5, 4), 5555);
        assert_eq!(repeat(123, 2), 123123);
    }

    #[test]
    fn test_split() {
        assert_eq!(split(11, 2), 1);
        assert_eq!(split(95, 2), 9);
        assert_eq!(split(998, 2), 10);
        assert_eq!(split(115, 2), 10);
        assert_eq!(split(11515, 2), 100);
        assert_eq!(split(1188511880, 2), 11885);

        assert_eq!(split(11515, 3), 10);
        assert_eq!(split(824824821, 3), 824);
        assert_eq!(split(2121212118, 5), 21);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
