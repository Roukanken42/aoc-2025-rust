use advent_of_code::utils::{Parsable, parse_input};
use itertools::Itertools;
use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, line_ending, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::cmp::Ordering;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Input {
    intervals: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl Parsable<'_> for Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_interval = separated_pair(u64::parse, char('-'), u64::parse);

        let (input, intervals) = separated_list1(line_ending, parse_interval).parse(input)?;
        let (input, _) = line_ending.parse(input)?;
        let (input, _) = line_ending.parse(input)?;
        let (input, ingredients) = separated_list1(newline, u64::parse).parse(input)?;

        Ok((
            input,
            Self {
                intervals,
                ingredients,
            },
        ))
    }
}

fn parse(input: &str) -> IResult<&str, Input> {
    parse_input(Input::parse).parse(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let result = input
        .ingredients
        .iter()
        .filter(|&&i| input.intervals.iter().any(|&(a, b)| a <= i && i <= b))
        .count();

    Some(result)
}

#[derive(Debug, PartialEq, Eq, Ord)]
enum Broom {
    Start(u64),
    End(u64),
}

impl Broom {
    fn value(&self) -> u64 {
        match self {
            Self::Start(v) => *v,
            Self::End(v) => *v,
        }
    }
}

impl PartialOrd for Broom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, input) = parse(input).unwrap();

    let result = input
        .intervals
        .iter()
        .flat_map(|&(start, end)| [Broom::Start(start), Broom::End(end + 1)])
        .sorted()
        .scan((0, 0), |(last, count), broom| {
            let size = if *count > 0 { broom.value() - *last } else { 0 };

            *last = broom.value();
            *count += match broom {
                Broom::Start(_) => 1,
                Broom::End(_) => -1,
            };

            Some(size)
        })
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

        assert!(result.is_ok());
        let (_, input) = result.unwrap();

        assert_eq!(input.intervals.len(), 4);
        assert_eq!(input.ingredients.len(), 6);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
