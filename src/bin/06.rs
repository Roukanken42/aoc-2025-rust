use advent_of_code::utils::dynamic_zip::DynamicZipable;
use advent_of_code::utils::{Parsable, parse_input, parse_input_by_lines};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
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

impl Operation {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operation::Add),
            '*' => Some(Operation::Multiply),
            _ => None,
        }
    }
}

impl Parsable<'_> for Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Operation::Add, char('+')),
            value(Operation::Multiply, char('*')),
        ))
        .parse(input)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Input {
    data: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl Parsable<'_> for Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_number_row = separated_list1(space1, u64::parse);

        let (input, _) = space0.parse(input)?;
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

fn parse_stupid_input(input: &str) -> IResult<&str, Vec<&str>> {
    parse_input_by_lines(take_while1(|c| c != '\n')).parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, input) = parse(input).unwrap();

    let result = input
        .operations
        .iter()
        .enumerate()
        .scan(0, |_, (index, operation)| {
            Some(match operation {
                Operation::Add => input.data.iter().fold(0, |acc, row| acc + row[index]),
                Operation::Multiply => input.data.iter().fold(1, |acc, row| acc * row[index]),
            })
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, input) = parse_stupid_input(input).unwrap();

    let columns = input.iter().map(|row| row.chars().rev()).dynamic_zip();

    let mut result = 0;
    let mut operands = vec![];
    let mut op = Operation::Multiply;

    for col in columns.chain(std::iter::once(vec![' '; input.len()])) {
        if col.iter().all(|c| *c == ' ') {
            result += match op {
                Operation::Add => operands.iter().fold(0, |acc, x| acc + x),
                Operation::Multiply => operands.iter().fold(1, |acc, x| acc * x),
            };

            operands.clear();
        }

        let number = col
            .iter()
            .filter_map(|c| ('0'..='9').contains(c).then(|| *c as u64 - '0' as u64))
            .fold(0, |acc, n| acc * 10 + n);

        let operation = col.iter().filter_map(|c| Operation::from_char(*c)).next();

        if number != 0 {
            operands.push(number);
        }

        if let Some(o) = operation {
            op = o;
        }
    }

    Some(result)
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
