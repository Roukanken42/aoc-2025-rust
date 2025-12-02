use advent_of_code::utils::{Parsable, parse_input};
use nom::character::char;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use std::cmp::max;
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

fn nearest_invalid_base_upwards(number: u64) -> u64 {
    let length = number.ilog10() + 1;
    let first_half = number.div(10u64.pow((length + 1) / 2));

    println!(
        "{} {} {} {}",
        number,
        length,
        first_half,
        double(first_half)
    );

    if double(first_half) >= number {
        first_half
    } else {
        first_half + 1
    }
}

fn count_invalid_in_interval(interval: &Interval) -> u64 {
    let low = nearest_invalid_base_upwards(interval.start);
    let high = nearest_invalid_base_upwards(interval.end + 1);

    max(high - low, 0)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, data) = parse(input).unwrap();

    let result = data
        .into_iter()
        .map(|interval| count_invalid_in_interval(&interval))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(nearest_invalid_base_upwards(11), 1);
        assert_eq!(nearest_invalid_base_upwards(95), 9);
        assert_eq!(nearest_invalid_base_upwards(998), 10);
        assert_eq!(nearest_invalid_base_upwards(1188511880), 11885);
    }

    #[test]
    fn test_count_invalid_in_interval() {
        assert_eq!(count_invalid_in_interval(&Interval::new(11, 22)), 2);
        assert_eq!(count_invalid_in_interval(&Interval::new(11, 1500)), 14);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
