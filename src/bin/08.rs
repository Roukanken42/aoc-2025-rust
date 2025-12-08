use advent_of_code::utils::location3d::{Location3, location3};
use advent_of_code::utils::parse_input_by_lines;
use nom::IResult;
use nom::Parser;
use nom::character::complete::char;
use nom::error::Error;

advent_of_code::solution!(8);

fn parse(input: &str) -> IResult<&str, Vec<Location3<u32>>, Error<&str>> {
    parse_input_by_lines(location3(char(','))).parse(input)
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
                    Location3::new(162, 817, 812),
                    Location3::new(57, 618, 57),
                    Location3::new(906, 360, 560),
                    Location3::new(592, 479, 940),
                    Location3::new(352, 342, 300),
                    Location3::new(466, 668, 158),
                    Location3::new(542, 29, 236),
                    Location3::new(431, 825, 988),
                    Location3::new(739, 650, 466),
                    Location3::new(52, 470, 668),
                    Location3::new(216, 146, 977),
                    Location3::new(819, 987, 18),
                    Location3::new(117, 168, 530),
                    Location3::new(805, 96, 715),
                    Location3::new(346, 949, 466),
                    Location3::new(970, 615, 88),
                    Location3::new(941, 993, 340),
                    Location3::new(862, 61, 35),
                    Location3::new(984, 92, 344),
                    Location3::new(425, 690, 689),
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
