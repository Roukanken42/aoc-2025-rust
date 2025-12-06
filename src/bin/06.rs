use advent_of_code::utils::{Parsable, parse_input};
use nom::branch::alt;
use nom::character::char;
use nom::character::complete::{line_ending, space0, space1};
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::{IResult, Parser};

advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Input {
    data: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl Parsable<'_> for Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_number_row = separated_list1(space1, u64::parse);

        let (input, data) =
            separated_list1((space0, line_ending, space0), parse_number_row).parse(input)?;
        let (input, _) = (space0, line_ending, space0).parse(input)?;
        let (input, operations) = separated_list1(
            space1,
            alt((
                value(Operation::Add, char('+')),
                value(Operation::Multiply, char('*')),
            )),
        )
        .parse(input)?;

        let (input, _) = space0.parse(input)?;

        Ok((input, Self { data, operations }))
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
    use crate::Operation::{Add, Multiply};

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse(&input);

        assert_eq!(
            result,
            Ok((
                "",
                Input {
                    data: vec![
                        vec![123, 328, 51, 64],
                        vec![45, 64, 387, 23],
                        vec![6, 98, 215, 314]
                    ],
                    operations: vec![Multiply, Add, Multiply, Add]
                }
            ))
        );
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
