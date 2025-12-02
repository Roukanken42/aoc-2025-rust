use advent_of_code::utils::{Parsable, parse_input};
use nom::character::char;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

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
    fn parse_input() {
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
