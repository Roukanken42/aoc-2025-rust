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

pub fn part_one_brute(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let result = input
        .ingredients
        .iter()
        .filter(|&&i| input.intervals.iter().any(|&(a, b)| a <= i && i <= b))
        .count();

    Some(result)
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
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
        match (self, other) {
            (Self::Start(s), Self::End(o)) if *s == *o => Some(Ordering::Less),
            (Self::End(s), Self::Start(o)) if *s == *o => Some(Ordering::Greater),
            _ => self.value().partial_cmp(&other.value()),
        }
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

pub fn part_one(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let events = input
        .intervals
        .iter()
        .flat_map(|&(start, end)| [Broom::Start(start), Broom::End(end + 1)])
        .sorted()
        .scan(0, |count, broom| {
            let last_count = *count;

            *count += match broom {
                Broom::Start(_) => 1,
                Broom::End(_) => -1,
            };

            Some(match (last_count, *count) {
                (0, _) => Some(Broom::Start(broom.value())),
                (_, 0) => Some(Broom::End(broom.value())),
                _ => None,
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    fn bin_search(events: &[Broom], target: u64) -> Option<Broom> {
        if events.is_empty() {
            return None;
        }
        if events.len() == 1 && events[0].value() <= target {
            return Some(events[0]);
        };

        let mid = events.len() / 2;
        let (left, right) = events.split_at(mid);

        if events[mid].value() <= target {
            bin_search(right, target)
        } else {
            bin_search(left, target)
        }
    }

    let result = input
        .ingredients
        .iter()
        .map(|&i| bin_search(&events, i))
        .filter(|broom| {
            if let Some(Broom::Start(_)) = broom {
                true
            } else {
                false
            }
        })
        .count();

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
    fn test_part_one_brute() {
        let result = part_one_brute(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
