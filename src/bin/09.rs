use advent_of_code::utils::location::{Location, location};
use advent_of_code::utils::parse_input_by_lines;
use nom::IResult;
use nom::Parser;
use nom::character::complete::char;

advent_of_code::solution!(9);

fn parse(input: &str) -> IResult<&str, Vec<Location<u64>>> {
    parse_input_by_lines(location(char(','))).parse(input)
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

        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Location::new(7, 1),
                    Location::new(11, 1),
                    Location::new(11, 7),
                    Location::new(9, 7),
                    Location::new(9, 5),
                    Location::new(2, 5),
                    Location::new(2, 3),
                    Location::new(7, 3),
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
