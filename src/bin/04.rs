use advent_of_code::utils::location::Access2d;
use advent_of_code::utils::parse_input_by_lines;
use nom::bytes::complete::take_while1;
use nom::{IResult, Parser};

advent_of_code::solution!(4);

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let parse_line = take_while1(|c| c == '.' || c == '@').map(|line: &str| line.chars().collect());
    parse_input_by_lines(parse_line).parse(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, map) = parse(input).unwrap();

    let result = map
        .iter_2d_keys()
        .filter(|loc| {
            map.get_2d(*loc) == Some(&'@')
                && loc
                    .neighbours()
                    .into_iter()
                    .filter(|&neigh| map.get_2d(neigh) == Some(&'@'))
                    .count()
                    < 4
        })
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
