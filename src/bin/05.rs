use advent_of_code::utils::{Parsable, parse_input};
use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, line_ending, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

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

pub fn part_one(input: &str) -> Option<u64> {
    None
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

        assert!(result.is_ok());
        let (_, input) = result.unwrap();

        assert_eq!(input.intervals.len(), 4);
        assert_eq!(input.ingredients.len(), 6);
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
