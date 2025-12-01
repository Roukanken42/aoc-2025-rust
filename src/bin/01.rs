use advent_of_code::utils::{Parsable, parse_input_by_lines};
use nom::IResult;
use nom::Parser;
use nom::character::complete::one_of;
use nom::sequence::pair;

advent_of_code::solution!(1);

#[derive(Debug, PartialEq)]
enum Rotation {
    Left(u32),
    Right(u32),
}

impl Parsable<'_> for Rotation {
    fn parse(input: &str) -> IResult<&str, Self> {
        pair(one_of("LR"), u32::parse)
            .map(|(dir, value)| match dir {
                'L' => Rotation::Left(value),
                'R' => Rotation::Right(value),
                _ => unreachable!(),
            })
            .parse(input)
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Rotation>> {
    parse_input_by_lines(Rotation::parse).parse(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, rotations) = parse_input(input).unwrap();

    let result = rotations
        .into_iter()
        .scan(50i32, |acc, rot| {
            *acc = match rot {
                Rotation::Left(deg) => *acc - (deg as i32),
                Rotation::Right(deg) => *acc + (deg as i32),
            };

            Some(*acc)
        })
        .map(|x| x.rem_euclid(100))
        .filter(|x| *x == 0)
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, rotations) = parse_input(input).unwrap();

    let result = rotations
        .into_iter()
        .scan(50i32, |acc, rot| {
            let nextacc = match rot {
                Rotation::Left(deg) => *acc - (deg as i32),
                Rotation::Right(deg) => *acc + (deg as i32),
            };

            // This is definitely wrong
            let clicks = (acc.div_euclid(100) - nextacc.div_euclid(100)).abs();
            *acc = nextacc;

            Some(clicks)
        })
        .inspect(|x| println!("{:?}", x))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rotation::{Left, Right};

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse_input(&input);

        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Left(68),
                    Left(30),
                    Right(48),
                    Left(5),
                    Right(60),
                    Left(55),
                    Left(1),
                    Left(99),
                    Right(14),
                    Left(82),
                ]
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
